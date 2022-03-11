use std::{collections::HashMap, fs::File, io::Write};

use reqwest;
use scraper::Selector;
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::structs::{AppUser, Checkbox, ResponseSuccess, UserProfileGet};

pub async fn update_profile(user: AppUser, pool: &Pool<Postgres>) -> Result<(), anyhow::Error> {
    if let Some(profile_id) = &user.profile_id {
        let (title, image) = get_book_info(profile_id).await?;

        // Set currently reading title
        let _ = sqlx::query!("UPDATE users SET title = $1 WHERE id = $2", title, user.id)
            .execute(pool)
            .await?;

        // Download the cover if it hasn't already been saved
        let path = format!("covers/{}.jpg", title);
        if !std::path::Path::new(&path).exists() {
            let image = reqwest::get(image).await?.bytes().await?;
            let mut file = File::create(&path)?;
            let _ = file.write_all(&image);
        }

        if user.update_status || user.update_title {
            let client = reqwest::Client::new();
            let mut profile: HashMap<String, String> = HashMap::new();

            if user.update_status {
                // Status emoji will be reset if not provided when setting status text
                // So, set to current status emoji to avoid resetting
                let user_profile = get_user_profile(&user).await?;
                profile.insert(String::from("status_text"), title.clone());
                profile.insert(
                    String::from("status_emoji"),
                    user_profile.profile.status_emoji,
                );
            }

            if user.update_title {
                profile.insert(String::from("title"), title.clone());
            }

            let resp = client
                .post("https://slack.com/api/users.profile.set")
                .header("Authorization", format!("Bearer {}", &user.access_token))
                .json(&json!({ "profile": profile }))
                .send()
                .await?;

            // Request can "succeed" with a failure message
            let status: ResponseSuccess = resp.json().await?;
            if !status.ok {
                return Err(anyhow::anyhow!(format!(
                    "Unable to set profile: {}",
                    status.error
                )));
            }
        }

        if user.update_picture {
            // Using the file method on multipart form requires using the blocking client
            tokio::task::spawn_blocking(move || -> Result<(), anyhow::Error> {
                let client = reqwest::blocking::Client::new();
                let form = reqwest::blocking::multipart::Form::new().file("image", &path)?;

                let resp = client
                    .post("https://slack.com/api/users.setPhoto")
                    .header("Authorization", format!("Bearer {}", &user.access_token))
                    .multipart(form)
                    .send()?;

                // Request can "succeed" with a failure message
                let status: ResponseSuccess = resp.json()?;
                if !status.ok {
                    return Err(anyhow::anyhow!(format!(
                        "Unable to set photo: {}",
                        status.error
                    )));
                }
                Ok(())
            })
            .await??;
        }
    }
    Ok(())
}

// Gets information about user's profile, specifically to avoid resetting status emoji
async fn get_user_profile(user: &AppUser) -> Result<UserProfileGet, anyhow::Error> {
    let client = reqwest::Client::new();
    Ok(client
        .get("https://slack.com/api/users.profile.get")
        .header("Authorization", format!("Bearer {}", user.access_token))
        .send()
        .await?
        .json()
        .await?)
}

// Only gets information for the first book in the "Currently Reading" shelf
async fn get_book_info(profile_id: &str) -> Result<(String, String), anyhow::Error> {
    let currently_reading_selector = Selector::parse("#currentlyReadingReviews").unwrap();
    let title_selector = Selector::parse("a.bookTitle").unwrap();
    let image_selector = Selector::parse("img").unwrap();

    let response = reqwest::get(format!(
        "https://www.goodreads.com/user/show/{}",
        profile_id
    ))
    .await?
    .text()
    .await?;

    let document = scraper::Html::parse_document(&response);

    let currently_reading = document
        .select(&currently_reading_selector)
        .next()
        .ok_or(anyhow::anyhow!("Unable to find current book"))?;

    let title = currently_reading
        .select(&title_selector)
        .next()
        .ok_or(anyhow::anyhow!("Unable to find title"))?
        .inner_html();

    let img = currently_reading
        .select(&image_selector)
        .next()
        .ok_or(anyhow::anyhow!("Unable to find cover"))?
        .value()
        .attr("src")
        .ok_or(anyhow::anyhow!("Unable to find cover"))?;

    Ok((title, img.to_string()))
}

// Checkboxes return "on" when checked which needs to be converted to a bool
pub fn checkbox_to_bool(input: &Checkbox) -> bool {
    if let Some(val) = input {
        match val.as_str() {
            "on" => true,
            _ => false,
        }
    } else {
        false
    }
}

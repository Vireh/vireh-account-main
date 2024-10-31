use eyre::OptionExt;
use reqwest::{Client, ClientBuilder};
use scraper::{Html, Selector};
use serde_json::json;

use crate::utils;

/// Main function to handle email password update
pub async fn update_email_password(email: &str, current_password: &str) -> eyre::Result<String> {
    let client = ClientBuilder::new().cookie_store(true).build()?;
    let login_token = fetch_token(&client, "https://cock.li/login").await?;
    authenticate_user(&client, email, current_password, &login_token).await?;
    
    let change_password_token = fetch_token(&client, "https://cock.li/user/changepass").await?;
    let new_password = utils::generate_random_password();
    execute_password_change(&client, current_password, &new_password, &change_password_token).await?;
    
    Ok(new_password)
}

/// Handles password change with current and new passwords
async fn execute_password_change(
    client: &Client,
    current_password: &str,
    new_password: &str,
    token: &str,
) -> eyre::Result<()> {
    let payload = json!({
        "_token": token,
        "current_password": current_password,
        "password": new_password,
        "password_confirmation": new_password,
    });

    let response = client
        .post("https://cock.li/user/changepass")
        .form(&payload)
        .send()
        .await?;

    response.error_for_status()?;
    Ok(())
}

/// Authenticates the user with the provided credentials
async fn authenticate_user(client: &Client, email: &str, password: &str, token: &str) -> eyre::Result<()> {
    let payload = json!({
        "_token": token,
        "email": email,
        "password": password,
    });

    let response = client
        .post("https://cock.li/login")
        .form(&payload)
        .send()
        .await?;

    response.error_for_status()?;
    Ok(())
}

/// Fetches a hidden token from the provided URL
async fn fetch_token(client: &Client, url: &str) -> eyre::Result<String> {
    let response = client.get(url).send().await?;
    let html_content = response.text().await?;
    let document = Html::parse_document(&html_content);

    let selector = Selector::parse("input[type='hidden'][name='_token']")
        .map_err(|err| eyre::eyre!("Selector parsing error: {}", err))?;

    let token = document
        .select(&selector)
        .next()
        .ok_or_eyre("Token element not found")?
        .value()
        .attr("value")
        .ok_or_eyre("Token value not found")?
        .to_string();

    Ok(token)
}

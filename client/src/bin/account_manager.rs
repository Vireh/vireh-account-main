use std::{fs::File, io::Write};

use client::{cockli::encumber_email, utils::get_random_password};
use dotenv::dotenv;
use x_rs::account::{login, Account};

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let user_name = std::env::var("USER_NAME").expect("USER_NAME not set");
    let user_password = std::env::var("USER_PASSWORD").expect("USER_PASSWORD not set");
    let user_email = std::env::var("USER_EMAIL").expect("USER_EMAIL not set");
    let email_pass = std::env::var("EMAIL_PASS").expect("EMAIL_PASS not set");
    let totp_code = std::env::var("USER_TOTP").ok();
    
    log::info!("Username: {}", user_name);
    log::info!("Password: {}", user_password);

    let mut user_login = login::Login::new(user_name, user_password.clone(), user_email.clone(), totp_code, None)
        .expect("Failed to create login instance");
    let auth_data = user_login.login().await.expect("Login failed");

    let mut user_account = Account::from_auth(auth_data).expect("Failed to create account from auth");
    let new_user_password = get_random_password();
    
    user_account.change_password(&user_password, &new_user_password)
        .await
        .expect("Failed to change password");
    log::info!("Password changed to: {}", new_user_password);
    
    user_account.refresh_cookies().await.expect("Failed to refresh cookies");

    let oauth_apps = user_account.get_all_oauth_applications().await.expect("Failed to get OAuth applications");
    let filtered_apps: Vec<_> = oauth_apps
        .into_iter()
        .filter(|app| app.app_id != "29459355")
        .collect();
    
    for app in filtered_apps.iter() {
        user_account.revoke_oauth_application(&app.token)
            .await
            .expect("Failed to revoke OAuth application");
    }

    let email_phone_info = user_account.get_email_phone_info().await.expect("Failed to get email and phone info");
    assert!(email_phone_info.emails.len() == 1);
    assert!(email_phone_info.emails[0].email == user_email);
    assert!(email_phone_info.phone_numbers.is_empty());

    let new_email_pass = encumber_email(&user_email, &email_pass).await.expect("Failed to encumber email password");
    log::info!("Email password changed to: {}", new_email_pass);

    let env_file_path = "config.env";
    let mut env_file = File::create(env_file_path).expect("Failed to create file");

    writeln!(env_file, "USER_PASSWORD={}", new_user_password).expect("Failed to write user password");
    writeln!(env_file, "EMAIL_PASS={}", new_email_pass).expect("Failed to write email password");

    let cookies = user_account.auth_cookie_string();
    let cookies_json = serde_json::to_string(&cookies).expect("Failed to serialize account cookies");
    writeln!(env_file, "AUTH_TOKENS={}", cookies_json).expect("Failed to write account cookies");

    log::info!("Credentials saved to {}", env_file_path);
}

use serde::{Deserialize, Serialize};
use super::builder::TwitterClient;

#[derive(Debug, Deserialize)]
struct UserInfoResponse {
    data: UserInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub username: String,
    pub profile_image_url: String,
}

impl TwitterClient<'_> {
    pub async fn get_user_info(&self) -> eyre::Result<UserInfo> {
        let url = "https://api.twitter.com/2/users/me?user.fields=profile_image_url,most_recent_tweet_id";
        
        let resp = self.client
            .get(url)
            .send()
            .await?;

        // Check for a successful response status
        if !resp.status().is_success() {
            let error_message = resp.text().await?;
            log::error!("Failed to fetch user info: {}", error_message);
            eyre::bail!(error_message);
        }

        let user_info: UserInfoResponse = resp.json().await?;
        log::info!("Fetched user info: {:?}", user_info.data);
        
        Ok(user_info.data)
    }
}

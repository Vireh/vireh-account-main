use serde::Serialize;
use super::builder::TwitterClient;

#[derive(Debug, Serialize)]
struct LikeRequest {
    tweet_id: String,
}

impl TwitterClient<'_> {
    pub async fn like_tweet(&self, user_id: String, tweet_id: String) -> eyre::Result<()> {
        let like_request = LikeRequest { tweet_id }; // Create a like request

        self.client
            .post(format!("https://api.twitter.com/2/users/{}/likes", user_id))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&like_request)?) // Serialize the like request
            .send()
            .await?; // Send the request

        Ok(())
    }

    pub async fn retweet_tweet(&self, user_id: String, tweet_id: String) -> eyre::Result<()> {
        let retweet_request = LikeRequest { tweet_id }; // Create a retweet request

        self.client
            .post(format!("https://api.twitter.com/2/users/{}/retweets", user_id))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&retweet_request)?) // Serialize the retweet request
            .send()
            .await?; // Send the request

        Ok(())
    }
}

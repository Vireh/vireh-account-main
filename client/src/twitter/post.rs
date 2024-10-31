use serde::Deserialize;
use super::{builder::TwitterClient, tweet::Tweet};

#[derive(Debug, Deserialize)]
struct TweetData {
    id: String,
}

#[derive(Debug, Deserialize)]
struct TweetResponse {
    data: TweetData,
}

#[derive(Deserialize, Debug)]
struct MediaUploadResult {
    media_id_string: String,
}

impl TwitterClient<'_> {
    pub async fn post_tweet(&self, tweet: Tweet) -> eyre::Result<String> {
        tweet.validate()?; // Validate the tweet
        let tweet_json = serde_json::to_string(&tweet)?; // Convert tweet to JSON

        let response = self.client
            .post("https://api.twitter.com/2/tweets")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(tweet_json)
            .send()
            .await?;

        let response_body = response.text().await?;
        let tweet_response: Result<TweetResponse, _> = serde_json::from_str(&response_body);

        match tweet_response {
            Ok(response) => {
                log::info!("Tweet response: {:?}", response);
                Ok(response.data.id)
            }
            Err(err) => {
                log::error!("Failed to decode tweet response: {:?}, body: {}", err, response_body);
                Err(eyre::eyre!("Failed to decode tweet response"))
            }
        }
    }

    pub async fn upload_media(&self, media_bytes: Vec<u8>, additional_owners: Option<Vec<String>>) -> eyre::Result<String> {
        let mut multipart_form = reqwest::multipart::Form::new()
            .part("media", reqwest::multipart::Part::bytes(media_bytes));

        if let Some(owners) = additional_owners {
            multipart_form = multipart_form.text("additional_owners", owners.join(","));
        }

        let upload_response = self.client
            .post("https://upload.twitter.com/1.1/media/upload.json")
            .multipart(multipart_form)
            .send()
            .await?;

        let upload_response_body = upload_response.text().await?;
        let media_upload_result: Result<MediaUploadResult, _> = serde_json::from_str(&upload_response_body);

        match media_upload_result {
            Ok(result) => {
                log::info!("Media upload response: {:?}", result);
                Ok(result.media_id_string)
            }
            Err(err) => {
                log::error!("Failed to decode media upload response: {:?}, body: {}", err, upload_response_body);
                Err(eyre::eyre!("Failed to decode media upload response"))
            }
        }
    }
}

use oauth1_request::signature_method::hmac_sha1::HmacSha1;
use reqwest_oauth1::{Client, OAuthClientProvider, Secrets, Signer};
use eyre::Result;
use super::auth::{self, TwitterTokenPair};

#[derive(Debug, Clone)]
pub struct TwitterConfig {
    consumer_key: String,
    consumer_secret: String,
}

pub struct TwitterClient<'a> {
    client: Client<Signer<'a, Secrets<'a>, HmacSha1>>,
}

impl TwitterConfig {
    pub fn new(consumer_key: String, consumer_secret: String) -> Self {
        Self { consumer_key, consumer_secret }
    }

    pub async fn request_oauth_token(&self, callback_url: String) -> Result<TwitterTokenPair> {
        auth::request_oauth_token(
            self.consumer_key.clone(),
            self.consumer_secret.clone(),
            callback_url,
        ).await
    }

    pub async fn authorize_token(
        &self,
        oauth_token: String,
        oauth_token_secret: String,
        oauth_verifier: String,
    ) -> Result<TwitterTokenPair> {
        auth::authorize_token(
            self.consumer_key.clone(),
            self.consumer_secret.clone(),
            oauth_token,
            oauth_token_secret,
            oauth_verifier,
        ).await
    }

    pub fn create_client(&self, tokens: TwitterTokenPair) -> TwitterClient<'_> {
        let secrets = Secrets::new(self.consumer_key.clone(), self.consumer_secret.clone())
            .token(tokens.token.clone(), tokens.secret.clone());

        let client = reqwest::Client::new().oauth1(secrets);
        TwitterClient { client }
    }
}

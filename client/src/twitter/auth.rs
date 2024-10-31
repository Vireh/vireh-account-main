use reqwest::Client;
use serde::{Deserialize, Serialize};
use eyre::Result;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct RequestTokenParams {
    oauth_callback: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct RequestTokenResponse {
    oauth_token: String,
    oauth_token_secret: String,
    oauth_callback_confirmed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct CallbackQueryParams {
    oauth_token: String,
    oauth_verifier: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TwitterOAuthToken {
    pub token: String,
    pub secret: String,
}

pub struct TwitterOAuthClient {
    app_key: String,
    app_secret: String,
    http_client: Client,
}

impl TwitterOAuthClient {
    pub fn new(app_key: String, app_secret: String) -> Self {
        let http_client = Client::new();
        Self {
            app_key,
            app_secret,
            http_client,
        }
    }

    pub async fn request_oauth_token(&self, callback_url: &str) -> Result<TwitterOAuthToken> {
        let params = RequestTokenParams {
            oauth_callback: callback_url.to_string(),
        };

        let response = self.http_client
            .post("https://api.twitter.com/oauth/request_token")
            .sign(self.app_key.clone(), self.app_secret.clone())
            .query(&params)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn authorize_token(
        &self,
        oauth_token: &str,
        oauth_token_secret: &str,
        oauth_verifier: &str,
    ) -> Result<TwitterOAuthToken> {
        let query_params = AccessTokenQueryParams {
            oauth_verifier: oauth_verifier.to_string(),
        };

        let secrets = reqwest_oauth1::Secrets::new(self.app_key.clone(), self.app_secret.clone())
            .token(oauth_token.to_string(), oauth_token_secret.to_string());

        let response = self.http_client
            .post("https://api.twitter.com/oauth/access_token")
            .sign(secrets)
            .query(&query_params)
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn handle_response(&self, response: reqwest::Response) -> Result<TwitterOAuthToken> {
        let status = response.status();
        if !status.is_success() {
            eyre::bail!(response.text().await?);
        }

        let response_bytes = response.bytes().await?;
        let response_body: RequestTokenResponse = serde_urlencoded::from_bytes(&response_bytes)?;

        if !response_body.oauth_callback_confirmed {
            eyre::bail!("Callback URL not confirmed.");
        }

        Ok(TwitterOAuthToken {
            token: response_body.oauth_token,
            secret: response_body.oauth_token_secret,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct AccessTokenQueryParams {
    oauth_verifier: String,
} 

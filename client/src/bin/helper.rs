use std::sync::Arc;
use axum::{
    extract::{Query, State},
    response::Redirect,
};
use serde::Deserialize;
use tokio::sync::{oneshot, Mutex};
use tower_http::cors::CorsLayer;
use client::twitter::{auth::TwitterTokenPair, builder::TwitterBuilder};
use std::io::Write;

#[derive(Clone)]
pub struct AppState {
    base_url: String,
    twitter_builder: TwitterBuilder,
    twitter_token_pair: Arc<Mutex<Option<TwitterTokenPair>>>,
    shutdown_sender: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

#[derive(Deserialize)]
pub struct AuthCallbackQuery {
    oauth_token: String,
    oauth_verifier: String,
}

pub async fn initiate_login(State(app_state): State<AppState>) -> Redirect {
    let callback_url = format!("{}/callback", app_state.base_url);
    log::info!("Initiating login");
    
    let oauth_tokens = app_state
        .twitter_builder
        .request_oauth_token(callback_url)
        .await
        .expect("Failed to request OAuth token");

    let mut token_pair_lock = app_state.twitter_token_pair.lock().await;
    *token_pair_lock = Some(oauth_tokens.clone());

    let redirect_url = format!(
        "https://api.twitter.com/oauth/authenticate?oauth_token={}",
        oauth_tokens.token
    );
    log::info!("Redirecting to {}", &redirect_url);
    Redirect::temporary(&redirect_url)
}

pub async fn handle_callback(
    State(app_state): State<AppState>,
    Query(query): Query<AuthCallbackQuery>,
) -> String {
    let oauth_token = query.oauth_token;
    let oauth_verifier = query.oauth_verifier;
    log::info!("Handling callback");

    let twitter_token_pair = app_state
        .twitter_token_pair
        .lock()
        .await
        .clone()
        .expect("No token pair found");

    assert_eq!(oauth_token, twitter_token_pair.token);

    let token_pair = app_state
        .twitter_builder
        .authorize_token(
            twitter_token_pair.token,
            twitter_token_pair.secret,
            oauth_verifier,
        )
        .await
        .expect("Failed to authorize token");

    let mut token_pair_lock = app_state.twitter_token_pair.lock().await;
    *token_pair_lock = Some(token_pair.clone());

    let twitter_client = app_state.twitter_builder.with_auth(token_pair);
    let user_info = twitter_client
        .get_user_info()
        .await
        .expect("Failed to get user info");

    if let Some(sender) = app_state.shutdown_sender.lock().await.take() {
        let _ = sender.send(());
    }

    format!("Successfully logged into {}", user_info.name)
}

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("Starting application");

    let base_url = "http://127.0.0.1:4000".to_string();
    let consumer_key = std::env::var("TWITTER_CONSUMER_KEY").expect("TWITTER_CONSUMER_KEY not set");
    let consumer_secret = std::env::var("TWITTER_CONSUMER_SECRET").expect("TWITTER_CONSUMER_SECRET not set");

    let twitter_builder = TwitterBuilder::new(consumer_key, consumer_secret);

    let (shutdown_sender, shutdown_receiver) = oneshot::channel();
    let app_state = AppState {
        base_url,
        twitter_builder: twitter_builder.clone(),
        twitter_token_pair: Arc::new(Mutex::new(None)),
        shutdown_sender: Arc::new(Mutex::new(Some(shutdown_sender))),
    };

    let app = axum::Router::new()
        .route("/login", axum::routing::get(initiate_login))
        .route("/callback", axum::routing::get(handle_callback))
        .layer(CorsLayer::permissive())
        .with_state(app_state.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let server = axum::serve(listener, app);
    server
        .with_graceful_shutdown(async {
            shutdown_receiver.await.ok();
        })
        .await
        .ok();
    
    log::info!("Shutting down server after receiving credentials.");

    let tokens = app_state.twitter_token_pair.lock().await.take().expect("No tokens found");

    let mut env_file = std::fs::File::create("updated.env").expect("Failed to create env file");
    writeln!(env_file, "X_ACCESS_TOKEN={}", tokens.token).expect("Failed to write access token");
    writeln!(env_file, "X_ACCESS_TOKEN_SECRET={}", tokens.secret).expect("Failed to write access token secret");

    log::info!("Credentials written to updated.env");
}

pub mod auth;
pub mod builder;
pub mod info;
pub mod post;
pub mod react;
pub mod tweet;

pub fn get_callback_url(callback_base_url: String) -> String {
    format!("https://{}/callback?", callback_base_url)
}
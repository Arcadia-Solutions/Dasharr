pub mod helpers;
pub mod user_stats;

use once_cell::sync::Lazy;
use reqwest::Client;

pub static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent("dasharr")
        .build()
        .expect("Failed to create HTTP client")
});

use reqwest::Client;
use std::time::Duration;

pub fn create_client() -> Result<Client, reqwest::Error> {
    Client::builder()
        .user_agent("PeaceMaker-Scanner/1.0")
        .timeout(Duration::from_secs(10))
        .danger_accept_invalid_certs(true) // allow danger certs
        .build()
}

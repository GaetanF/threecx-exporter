pub mod client;
pub mod login;
pub mod status;
pub mod service_list;

use crate::ThreeCXConfiguration;
use reqwest;

#[derive(Debug)]
pub struct ThreeCXClient {
    pub configuration: ThreeCXConfiguration,
    client: reqwest::Client
}
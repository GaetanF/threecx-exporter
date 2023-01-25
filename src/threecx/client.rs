use super::ThreeCXClient;

use reqwest;
use reqwest::Url;
use serde::Serialize;
use log::*;
use crate::ThreeCXConfiguration;
use crate::errors::{
    ThreeCXError
};

impl ThreeCXClient {
    /*****************************************
    *            Client Method
    *****************************************/
    pub fn new(configuration: ThreeCXConfiguration) -> ThreeCXClient {
        ThreeCXClient {
            configuration,
            client: reqwest::Client::builder()
                             .cookie_store(true)
                             .build().expect("Unable to get client")
        }
    }

    pub fn get_base_url(&self) -> String {
        format!("https://{}/api", self.configuration.host)
    }

    pub fn get(&self, endpoint: String) -> Result<reqwest::RequestBuilder, ThreeCXError>
    {
        let endpoint  = format!("{}/{}", self.get_base_url(), endpoint);

        let result_url = Url::parse(endpoint.as_str());

        if result_url.is_err() {
            return Err(ThreeCXError::BackendError);
        }
        let url = result_url.ok().unwrap();

        Ok(self.client.get(url))
    }

    pub fn post<T>(&self, endpoint: String, content: T) -> Result<reqwest::RequestBuilder, ThreeCXError>
        where T: Serialize + std::fmt::Debug
    {
        let endpoint  = format!("{}/{}", self.get_base_url(), endpoint);

        let result_url = Url::parse(endpoint.as_str());

        if result_url.is_err() {
            return Err(ThreeCXError::BackendError);
        }
        let url = result_url.ok().unwrap();

        Ok(self.client
            .post(url)
            .json(&content))
    }
}
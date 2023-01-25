use std::collections::HashMap;
use warp::http::StatusCode;
use super::ThreeCXClient;
use crate::errors::{
    ThreeCXError
};

impl ThreeCXClient {
    /*****************************************
    *            Login Method
    *****************************************/
    pub async fn login(&self) -> Result<bool, ThreeCXError>
    {
        let mut content = HashMap::new();
        content.insert("Username", self.configuration.username.clone());
        content.insert("Password", self.configuration.password.clone());

        let client = self.post(format!("login"), content);
        let resp = client.ok().unwrap().send().await?;

        match resp.status() {
            StatusCode::OK => {
                Ok(true)
            },
            _ => {
                Ok(false)
            }
        }
    }
}
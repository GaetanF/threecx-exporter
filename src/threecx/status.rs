
use super::ThreeCXClient;

use warp::http::StatusCode;
use serde_json::json;
use log::*;
use crate::errors::{ThreeCXError, InfoMessage, ErrorMessage, ErrorMessageType};
use crate::model::*;

impl ThreeCXClient {
    /*****************************************
   *            Get Server Status 3CX
   *****************************************/
    pub async fn get_server_status(&self) -> Result<ThreeCXSystemStatusResponse, ThreeCXError>
    {
        let client = self.get(format!("SystemStatus"));
        let resp = client.ok().unwrap().send().await?;
        debug!("{:?}", resp);

        match resp.status() {
            StatusCode::OK => {
                let body = resp.json::<ThreeCXSystemStatusResponse>().await.map_err(|err| {
                    error!(
                        "{}",
                        json!(ErrorMessage {
                            r#type: ErrorMessageType::InternalError,
                            reason: format!("unexpected format {:?}", err),
                            infos: InfoMessage {
                              resource: String::from("response"),
                              service: String::from("threecx"),
                              action: String::from("get_system_status"),
                              scope: String::from("ThreeCXSystemStatusResponse"),
                              ..InfoMessage::default()
                            }
                        })
                    );
                    ThreeCXError::InternalError
                })?;
                Ok({
                    debug!("{:?}", body);
                    body
                })
            },
            _ => {
                let body = resp.text().await;
                error!(
                    "{}",
                    json!(ErrorMessage {
                        r#type: ErrorMessageType::BadRequest,
                        reason: format!("{:?}", body.as_ref().unwrap()),
                        infos: InfoMessage {
                          resource: String::from("response"),
                          service: String::from("threecx"),
                          action: String::from("get_system_status"),
                          scope: String::from("ThreeCXSystemStatusResponse"),
                          ..InfoMessage::default()
                        }
                    })
                );

                Err(ThreeCXError::InternalError)
            }
        }
    }
}
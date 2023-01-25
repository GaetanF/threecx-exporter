use serde::{Deserialize, Serialize};
use std::num::ParseIntError;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum ErrorMessageType {
    BadRequest,
    InterfaceContractError,
    InternalError,
    ParseError,
    RuntimeError,
    TimeOut,
    ItemNotFound,
    AlreadyExist,
}

impl std::fmt::Display for ErrorMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ErrorMessageType::BadRequest => write!(f, "bad request"),
            ErrorMessageType::InterfaceContractError => write!(f, "interface contract error"),
            ErrorMessageType::InternalError => write!(f, "internal error"),
            ErrorMessageType::ParseError => write!(f, "parse error"),
            ErrorMessageType::RuntimeError => write!(f, "runtime error"),
            ErrorMessageType::TimeOut => write!(f, "timeout"),
            ErrorMessageType::ItemNotFound => write!(f, "item not found "),
            ErrorMessageType::AlreadyExist => write!(f, "item already exist "),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InfoMessage {
    pub data: serde_json::value::Value,
    pub resource: String,
    pub scope: String,
    pub service: String,
    pub action: String,
}

impl Default for InfoMessage {
    fn default() -> Self {
        InfoMessage {
            data: serde_json::json!({}),
            resource: String::new(),
            scope: String::new(),
            service: String::new(),
            action: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorMessage {
    pub r#type: ErrorMessageType,
    pub reason: String,
    pub infos: InfoMessage,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ThreeCXError {
    IOError(std::io::Error),
    HttpRequestError(reqwest::Error),
    BadRequest,
    NotFound,
    ItemNotFound(String),
    InternalError,
    TimeoutError,
    BackendError,
    FrontendError,
    AlreadyExist,
}

impl std::fmt::Display for ThreeCXError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ThreeCXError::IOError(ref err) => write!(f, "IO error: {}", err),
            ThreeCXError::BadRequest => write!(f, "bad request"),
            ThreeCXError::HttpRequestError(ref err) => write!(f, "http request error: {}", err),
            ThreeCXError::NotFound => write!(f, "not found"),
            ThreeCXError::ItemNotFound(ref _err) => write!(f, "not found: {}", _err),
            ThreeCXError::AlreadyExist => write!(f, "item already exist"),
            ThreeCXError::InternalError => write!(f, "internal error"),
            ThreeCXError::TimeoutError => write!(f, "time out"),
            ThreeCXError::BackendError => write!(f, "netnetwo backend error"),
            ThreeCXError::FrontendError => write!(f, "netnetwo frontend error"),
        }
    }
}

impl std::error::Error for ThreeCXError {
    fn cause(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ThreeCXError::IOError(ref err) => Some(err),
            ThreeCXError::BadRequest => None,
            ThreeCXError::HttpRequestError(ref err) => Some(err),
            ThreeCXError::NotFound => None,
            ThreeCXError::ItemNotFound(ref _err) => None,
            ThreeCXError::AlreadyExist => None,
            ThreeCXError::InternalError => None,
            ThreeCXError::TimeoutError => None,
            ThreeCXError::BackendError => None,
            ThreeCXError::FrontendError => None,
        }
    }
}

impl From<ParseIntError> for ThreeCXError {
    fn from(_err: ParseIntError) -> ThreeCXError {
        ThreeCXError::InternalError
    }
}

impl From<std::io::Error> for ThreeCXError {
    fn from(err: std::io::Error) -> ThreeCXError {
        ThreeCXError::IOError(err)
    }
}

impl From<reqwest::Error> for ThreeCXError {
    fn from(err: reqwest::Error) -> ThreeCXError {
        ThreeCXError::HttpRequestError(err)
    }
}

/*/// Error type for when only unexpected failures can happen
#[derive(Debug)]
#[allow(dead_code)]
pub enum UnexpectedError {
    InternalServerError,
}

impl std::fmt::Display for UnexpectedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.status_code().as_str())
    }
}
*/
/*/// Error type for single item operations
#[derive(Debug)]
#[allow(dead_code)]
pub enum ShowError {
    InternalServerError,
    NotFound,
}

impl std::fmt::Display for ShowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.status_code().as_str())
    }
}*/

/*/// Error type for creation operations
#[derive(Debug)]
#[allow(dead_code)]
pub enum CreateError {
    InternalServerError,
    BadRequest,
}

impl std::fmt::Display for CreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.status_code().as_str())
    }
}*/

/*/// Error type for modification operations
#[derive(Debug)]
#[allow(dead_code)]
pub enum EditError {
    InternalServerError,
    BadRequest,
    NotFound,
}


impl std::fmt::Display for EditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.status_code().as_str())
    }
}*/

/*/// Error type for events ingestion
#[derive(Debug)]
#[allow(dead_code)]
pub enum IngestError {
    InternalServerError,
    BadRequest,
    Forbidden,
    InvalidPayload,
    InvalidPayloadContentType,
    InvalidMetadata,
    InvalidLabels,
    Conflict,
}

impl std::fmt::Display for IngestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.status_code().as_str())
    }
}*/
use super::super::service::toornament;

use serde::Serialize;
use rocket::{
    http::{ContentType, Status},
    request::Request,
    response::{Response, Responder}
};
use std::io::Cursor;

#[derive(Serialize)]
pub struct Message {
    pub message: String
}

impl From<String> for Message {
    fn from(string: String) -> Self {
        Message {
            message: string
        }
    }
}

impl<'s> From<&'s str> for Message {
    fn from(string: &'s str) -> Self {
        Message {
            message: String::from(string)
        }
    }
}

pub struct JsonResponse<T> {
    #[serde(skip_serializing)]
    pub status: Status,
    pub response: T
}

#[derive(Debug, Serialize)]
pub struct CustomError {
    #[serde(skip_serializing)]
    pub status: Status,
    pub code: String,
    pub message: String
}

impl From<jsonwebtoken::errors::Error> for CustomError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        CustomError {
            status: Status::InternalServerError,
            code: "jwt_error".to_string(),
            message: "".to_string()
        }
    }
}

impl From<base64::DecodeError> for CustomError {
    fn from(error: base64::DecodeError) -> Self {
        CustomError {
            status: Status::InternalServerError,
            code: "internal_error".to_string(),
            message: error.to_string()
        }
    }
}

impl From<std::option::NoneError> for CustomError {
    fn from(error: std::option::NoneError) -> Self {
        CustomError {
            status: Status::Unauthorized,
            code: "invalid_token".to_string(),
            message: "".to_string()
        }
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(error: serde_json::Error) -> Self {
        CustomError {
            status: Status::InternalServerError,
            code: "internal_error".to_string(),
            message: error.to_string()
        }
    }
}

impl From<toornament::Error> for CustomError {
    fn from(error: toornament::Error) -> Self {
        CustomError {
            status: Status::InternalServerError,
            code: "toornament_error".to_string(),
            message: error.message
        }
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(error: reqwest::Error) -> Self {
        CustomError {
            status: Status::InternalServerError,
            code: "reqwest_error".to_string(),
            message: "".to_string()
        }
    }
}

impl From<reqwest::header::InvalidHeaderValue> for CustomError {
    fn from(error: reqwest::header::InvalidHeaderValue) -> Self {
        CustomError {
            status: Status::InternalServerError,
            code: "reqwest_error".to_string(),
            message: "".to_string()
        }
    }
}

impl From<reqwest::header::InvalidHeaderName> for CustomError {
    fn from(error: reqwest::header::InvalidHeaderName) -> Self {
        CustomError {
            status: Status::InternalServerError,
            code: "reqwest_error".to_string(),
            message: "".to_string()
        }
    }
}

impl<'r> Responder<'r> for CustomError {
    fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {
        let json = match serde_json::to_string(&self) {
            Ok(object) => object,
            Err(_) => return Err(Status::InternalServerError)
        };

        Response::build()
            .status(self.status)
            .header(ContentType::JSON)
            .sized_body(Cursor::new(json))
            .ok()
    }
}

impl<'r, T: Serialize> Responder<'r> for JsonResponse<T> {
    fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {
        let json = match serde_json::to_string(&self.response) {
            Ok(object) => object,
            Err(_) => return Err(Status::InternalServerError)
        };

        Response::build()
            .status(self.status)
            .header(ContentType::JSON)
            .sized_body(Cursor::new(json))
            .ok()
    }
}


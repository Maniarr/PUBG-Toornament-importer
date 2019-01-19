use super::super::service::toornament;

use jsonwebtoken::{encode, decode, Header, Validation};
use rocket::{
    http::{ContentType, Status},
    Outcome,
    request::{self, Request, FromRequest}
};
use chrono::{Duration, Utc};
use serde::{
    Serialize, Serializer,
    Deserialize, Deserializer,
    de::{self, Visitor, Unexpected}
};
use std::{
    fmt,
    convert::{self, TryFrom},
    io::Write
};

lazy_static!{
    pub static ref JWT_SECRET: Vec<u8> = std::env::var("JWT_SECRET").unwrap().into_bytes();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat : usize,
    pub exp: usize
}

impl<'a, 'r> FromRequest<'a, 'r> for Claims {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let jwt: String = request.headers().get("x-api-key").collect();
        let toornament_token: String = request.headers().get("x-toornament-api-key").collect();

        match verify_jwt(jwt, toornament_token) {
            Ok(raw_jwt) => Outcome::Success(raw_jwt.claims),
            Err(_error) => Outcome::Failure((Status::Unauthorized, "".to_owned()))
        }
    }
}

impl From<jsonwebtoken::errors::Error> for toornament::Error {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        toornament::Error {
            error: "JWT".to_string(),
            hint: "".to_string(),
            message: error.to_string()
        }
    }
}

impl From<base64::DecodeError> for toornament::Error {
    fn from(error: base64::DecodeError) -> Self {
        toornament::Error {
            error: "JWT".to_string(),
            hint: "".to_string(),
            message: error.to_string()
        }
    }
}

impl From<std::option::NoneError> for toornament::Error {
    fn from(error: std::option::NoneError) -> Self {
        toornament::Error {
            error: "JWT".to_string(),
            hint: "".to_string(),
            message: "invalid token".to_string()
        }
    }
}

impl From<serde_json::Error> for toornament::Error {
    fn from(error: serde_json::Error) -> Self {
        toornament::Error {
            error: "JWT".to_string(),
            hint: "".to_string(),
            message: error.to_string()
        }
    }
}

pub fn generate_jwt(access_token: String) -> Result<String, toornament::Error> {
    let informations = access_token.split(".").nth(1)?;

    let mut claims: Claims = serde_json::from_slice(&base64::decode(informations)?)?;

    claims.sub = access_token;

    match encode(&Header::default(), &claims, &*JWT_SECRET) {
        Ok(jwt) => Ok(jwt),
        Err(error) => Err(toornament::Error::from(error))
    }
}

pub fn verify_jwt(jwt: String, toornament_token: String) -> Result<jsonwebtoken::TokenData<Claims>, toornament::Error> {
    let validation = Validation { ..Validation::default() };

    let claims = decode::<Claims>(&jwt, &*JWT_SECRET, &validation)?;

    if claims.claims.sub == toornament_token {
        Ok(claims)
    } else {
        Err(toornament::Error {
            error: "".to_string(),
            hint: "".to_string(),
            message: "".to_string()
        })
    }
}

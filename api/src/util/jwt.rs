use super::super::{
    service::toornament,
    util::response::*
};

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

pub fn generate_jwt(access_token: String) -> Result<String, CustomError> {
    let informations = access_token.split(".").nth(1)?;

    let mut claims: Claims = serde_json::from_slice(&base64::decode(informations)?)?;

    claims.sub = access_token;

    match encode(&Header::default(), &claims, &*JWT_SECRET) {
        Ok(jwt) => Ok(jwt),
        Err(error) => Err(CustomError {
            status: Status::Unauthorized,
            code: "token_invalid".to_string(),
            message: "".to_string()
        })
    }
}

pub fn verify_jwt(jwt: String, toornament_token: String) -> Result<jsonwebtoken::TokenData<Claims>, CustomError> {
    let validation = Validation { ..Validation::default() };

    let claims = decode::<Claims>(&jwt, &*JWT_SECRET, &validation)?;

    if claims.claims.sub == toornament_token {
        Ok(claims)
    } else {
        Err(CustomError {
            status: Status::Unauthorized,
            code: "token_invalid".to_string(),
            message: "".to_string()
        })
    }
}

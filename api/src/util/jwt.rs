use super::super::model::user;

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

#[derive(Debug)]
pub enum Role {
    User
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::User      => "user".to_string()
        }
    }
}

impl TryFrom<String> for Role {
    type Error = fmt::Error;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        match name.as_str() {
            "user"  => Ok(Role::User),
            _       => Err(fmt::Error {})
        }
    }
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

struct RoleVisitor;

impl<'de> Visitor<'de> for RoleVisitor {
    type Value = Role;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "provided string isn't valid role")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        match Role::try_from(s.to_string()) {
            Ok(role)    => Ok(role),
            Err(_)      => Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        }
    }
}

impl<'de> Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(RoleVisitor {})
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub iss : String,
    pub exp: usize,
    pub roles: Vec<Role>
}

impl<'a, 'r> FromRequest<'a, 'r> for Claims {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let jwt: String = request.headers().get("x-api-key").collect();

        match verify_jwt(jwt) {
            Ok(raw_jwt) => Outcome::Success(raw_jwt.claims),
            Err(_error) => Outcome::Failure((Status::Unauthorized, "".to_owned()))
        }
    }
}

pub fn generate_jwt(user: user::User) -> Result<String, jsonwebtoken::errors::Error> {
    let my_claims = Claims {
        sub: user.id.unwrap(),
        iss: "ACME".to_owned(),
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        roles: user.roles
    };

    encode(&Header::default(), &my_claims, &*JWT_SECRET)
}

pub fn verify_jwt(token: String) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
    let validation = Validation { iss: Some("ACME".to_owned()), ..Validation::default() };

    decode::<Claims>(&token, &*JWT_SECRET, &validation)
}

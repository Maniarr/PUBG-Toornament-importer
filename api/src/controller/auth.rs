use super::super::{
    util::{jwt::*, response::*},
    service::toornament::{self, *},
};

use bcrypt::verify;
use rocket::{
    http::{ContentType, Status},
    Route
};
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize)]
struct Login {
    access_token: String
}

#[derive(Serialize, Deserialize)]
struct LoginLocation {
    connection_uri: String
}

#[options("/login")]
fn options_login() -> String {
    "".to_string()
}

#[get("/login")]
fn get_login() -> JsonResponse<LoginLocation, CustomError> {
    JsonResponse::Ok(Status::Ok.code, LoginLocation {
        connection_uri: toornament::get_connection_uri()
    })
}

#[derive(Serialize)]
struct Token {
    authentication_token: String,
    access_token: String,
    expires_in: i32,
    token_type: String,
    refresh_token: String,
    scope: String
}

#[post("/login", data="<login_request>")]
fn login(login_request: Json<LoginRequest>) -> JsonResponse<Token, toornament::Error> {
    let client = reqwest::Client::new();

    match get_tokens(&client, &login_request.into_inner()) {
        Ok(token) => {
            match generate_jwt(token.access_token.clone()) {
                Ok(jwt) => JsonResponse::Ok(Status::Ok.code, Token {
                    authentication_token: jwt,
                    access_token: token.access_token,
                    expires_in: token.expires_in,
                    token_type: token.token_type,
                    refresh_token: token.refresh_token,
                    scope: token.scope
                }),
                Err(error) => JsonResponse::Err(Status::Unauthorized.code, error)
            }
        },
        Err(error) => JsonResponse::Err(Status::Unauthorized.code, error)
    }
}

#[get("/me")]
fn me(jwt: Claims) -> Json<Claims> {
    Json(jwt)
}

pub fn register_routes() -> Vec<Route> {
    routes![
        options_login,
        get_login,
        login,
        me
    ]
}

use super::super::{
    model::{user},
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

#[post("/login", data="<login_request>")]
fn login(login_request: Json<LoginRequest>) -> JsonResponse<TokenResponse, toornament::Error> {
    let client = reqwest::Client::new();

    match get_tokens(&client, &login_request.into_inner()) {
        Ok(token) => JsonResponse::Ok(Status::Ok.code, token),
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
        login
    ]
}

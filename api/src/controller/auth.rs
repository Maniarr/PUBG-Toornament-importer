use super::super::{
    util::{jwt::*, response::*},
    service::toornament::{self, *},
};

use bcrypt::verify;
use rocket::{
    http::{Cookie, Cookies, ContentType, Status},
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

#[get("/login")]
fn get_login(mut cookies: Cookies) -> Result<JsonResponse<LoginLocation>, CustomError> {
    let csrf_token = uuid::Uuid::new_v4();

    cookies.add_private(Cookie::new("csrf_token", format!("{}", csrf_token)));

    Ok(JsonResponse {
        status: Status::Ok,
        response: LoginLocation {
            connection_uri: toornament::get_connection_uri(csrf_token)
        }
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

#[post("/login", data="<json_login_request>")]
fn login(mut cookies: Cookies, json_login_request: Json<LoginRequest>) -> Result<JsonResponse<Token>, CustomError> {
    let client = reqwest::Client::new();
    let login_request = json_login_request.into_inner();

    if let Some(cookie) = cookies.get_private("csrf_token") {
        if cookie.value() != login_request.state {
            return Err(CustomError {
                status: Status::Unauthorized,
                code: "invalid_cookie".to_string(),
                message: "Please clear your cache.".to_string()
            });
        }
    } else {
        return Err(CustomError {
            status: Status::Unauthorized,
            code: "invalid_csrf".to_string(),
            message: "Please clear your cache.".to_string()
        });
    }

    match get_tokens(&client, &login_request) {
        Ok(token) => {
            match generate_jwt(token.access_token.clone()) {
                Ok(jwt) => Ok(JsonResponse {
                    status: Status::Ok,
                    response: Token {
                        authentication_token: jwt,
                        access_token: token.access_token,
                        expires_in: token.expires_in,
                        token_type: token.token_type,
                        refresh_token: token.refresh_token,
                        scope: token.scope
                    }
                }),
                Err(error) => {
                    eprintln!("generate token: {:?}", error);

                    Err(error)
                }
            }
        },
        Err(error) => {
            eprintln!("{:?}", error);

            Err(error)
        }
    }
}

#[get("/me")]
fn me(jwt: Claims) -> Json<Claims> {
    Json(jwt)
}

pub fn register_routes() -> Vec<Route> {
    routes![
        get_login,
        login,
        me
    ]
}

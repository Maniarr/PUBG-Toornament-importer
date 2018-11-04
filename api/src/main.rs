extern crate actix_web;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate reqwest;
extern crate uuid;

use actix_web::middleware::Logger;
use actix_web::middleware::session::{self, RequestSession};
use actix_web::{server, App, http, HttpRequest, HttpResponse, Responder};

use uuid::Uuid;

use std::sync::{Arc, Mutex};
use std::env;

mod toornament;

use toornament::{ToornamentApi, ToornamentAuthorizer, TokenResponse};

struct AppState {
    toornament_api: Arc<Mutex<ToornamentApi>>
}

#[derive(Debug, Serialize)]
struct AppError {
    error: String
}

fn index(req: &HttpRequest<AppState>) -> impl Responder {
    let api = req.state().toornament_api.lock().unwrap();

    if let Ok(Some(TokenResponse::TokenResult { scope, token_type, expires_in, access_token, refresh_token })) = req.session().get::<TokenResponse>("toornament_tokens") {
        return HttpResponse::Ok().body(format!("access: {}\nrefresh: {}", access_token, refresh_token));
    }

    let csrf_token = Uuid::new_v4().to_hyphenated().to_string();

    req.session().set("csrf_token", &csrf_token);

    HttpResponse::Ok().body(format!("<a href=\"https://account.toornament.com/oauth2/authorize?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}\">Login with Toornament</a>", 
        api.client_id, 
        api.redirect_uri, 
        "organizer:view organizer:result",
        csrf_token
    ))
}

fn login(req: &HttpRequest<AppState>) -> impl Responder {
    let code = req.query().get("code").cloned();
    let state_token = req.query().get("state").cloned();
    let csrf_token = req.session().get::<String>("csrf_token").unwrap();

    if code.is_none() {
        return HttpResponse::Ok().body("No code provided by Toornament".to_string());
    }

    if csrf_token != state_token {
        return HttpResponse::Ok().body("CSRF token not valid".to_string());
    }

    let mut api = req.state().toornament_api.lock().unwrap();

    if let Ok(tokens) = api.authorize_with_code(code.unwrap()) {
        return match tokens {
            TokenResponse::TokenResult { scope, token_type, expires_in, access_token, refresh_token } => {
                req.session().set("toornament_tokens", TokenResponse::TokenResult {
                    scope: scope, token_type: token_type, expires_in: expires_in, access_token: access_token, refresh_token: refresh_token
                });

                return HttpResponse::Found()
                    .header(http::header::LOCATION, "/".to_string()).finish();
            },
            TokenResponse::TokenError { error, message, hint } => {
                return HttpResponse::Ok().body(format!("{}: {}", error, match hint {
                    Some(msg) => msg,
                    None => message
                }));
            }
        };
    }
        
    return HttpResponse::Ok().body("Error from application".to_string());
}

fn logout(req: &HttpRequest<AppState>) -> impl Responder {
    req.session().clear();

    return HttpResponse::Found()
        .header(http::header::LOCATION, "/".to_string()).finish();
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let toornament_api = Arc::new(Mutex::new(ToornamentApi {
        api_uri: env::var("TOORNAMENT_API_URI").expect("env TOORNAMENT_API_URI not found"),
        api_key: env::var("TOORNAMENT_API_KEY").expect("env TOORNAMENT_API_KEY not found"),
        client_id: env::var("TOORNAMENT_CLIENT_ID").expect("env TOORNAMENT_CLIENT_ID not found"),
        client_secret: env::var("TOORNAMENT_CLIENT_SECRET").expect("env TOORNAMENT_CLIENT_SECRET not found"),
        redirect_uri: env::var("REDIRECT_URI").expect("env REDIRECT_URI not found")
    }));

    server::new(move || App::with_state(AppState { toornament_api: toornament_api.clone() })
        .middleware(Logger::default())
        .middleware(session::SessionStorage::new(
                session::CookieSessionBackend::signed(&[0; 32]).secure(false)
        ))
        .resource("/", |r| r.f(index))
        .resource("/login", |r| r.f(login))
        .resource("/logout", |r| r.f(logout))
    ).bind(env::var("LISTEN_HOST").expect("env LISTEN_HOST not found"))
    .unwrap()
    .run();
}

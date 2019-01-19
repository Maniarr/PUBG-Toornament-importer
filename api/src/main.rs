#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]
#![feature(try_from)] // Rust 1.33 stable will be released on Fri Mar 01 2019.

extern crate bcrypt;
extern crate chrono;
extern crate jsonwebtoken;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate uuid;
extern crate dotenv;
extern crate reqwest;

mod controller;
mod util;
mod model;
mod service;

fn try_config() {
    let mut is_panic = false;
    let envs = vec![
        "JWT_SECRET",
        "PUBG_API_KEY",
        "TOORNAMENT_OAUTH_URI",
        "TOORNAMENT_API_URI",
        "TOORNAMENT_API_KEY",
        "TOORNAMENT_CLIENT_ID",
        "TOORNAMENT_CLIENT_SECRET",
        "REDIRECT_URI"
    ];

    for env in envs {
        match std::env::var(env) {
            Ok(_) => {},
            Err(_) => {
                eprintln!("{} is not defined", env);
                is_panic = true;
            }
        }
    }

    if is_panic {
        panic!("error on environment variables");
    }
}

fn main() {
    dotenv::dotenv();

    try_config();

    rocket::ignite()
        .mount("/", controller::auth::register_routes())
        .mount("/", controller::user::register_routes())
        .launch();
}

use super::super::{
    util::{jwt::*, response::*},
    service::pubg::{self},
};

use rocket::{
    http::{Cookie, Cookies, ContentType, Status},
    Route
};
use rocket_contrib::json::Json;

#[get("/tournaments")]
pub fn get_tournaments(_jwt: Claims) -> Result<JsonResponse<pubg::PubgResponse<Vec<pubg::Tournament>>>, CustomError> {
    let client = reqwest::Client::new();

    let tournaments = pubg::get_tournaments(&client)?;

    Ok(JsonResponse {
        status: Status::Ok,
        response: tournaments
    })
}

#[get("/tournaments/<tournament_id>")]
pub fn get_tournament(_jwt: Claims, tournament_id: String) -> Result<JsonResponse<pubg::PubgResponse<pubg::TournamentInfo>>, CustomError> {
    let client = reqwest::Client::new();

    Ok(JsonResponse {
        status: Status::Ok,
        response: pubg::get_tournament(&client, tournament_id)?
    })
}

#[get("/matches/<region>/<match_id>")]
pub fn get_match(_jwt: Claims, region: String, match_id: String) -> Result<JsonResponse<pubg::MatchResponse>, CustomError> {
    let client = reqwest::Client::new();

    Ok(JsonResponse {
        status: Status::Ok,
        response: pubg::get_match(&client, region, match_id)?
    })
}

#[get("/players?<username>&<platform>")]
pub fn get_players(_jwt: Claims, username: String, platform: String) -> Result<JsonResponse<pubg::PubgResponse<Vec<pubg::Player>>>, CustomError> {
    let client = reqwest::Client::new();

    Ok(JsonResponse {
        status: Status::Ok,
        response: pubg::get_players(&client, username, platform)?
    })
}

pub fn register_routes() -> Vec<Route> {
    routes![
        get_tournaments,
        get_tournament,
        get_match,
        get_players
    ]
}

use super::super::{
    util::{jwt::*, response::*},
    service::toornament,
};

use rocket::{
    http::{Cookie, Cookies, ContentType, Status},
    Route
};
use rocket_contrib::json::Json;

#[get("/tournaments")]
pub fn get_tournaments(jwt: Claims) -> Result<JsonResponse<Vec<toornament::Tournament>>, CustomError> {
    let client = reqwest::Client::new();

    let tournaments = toornament::get_tournaments(&client, jwt.sub)?;

    Ok(JsonResponse {
        status: Status::Ok,
        response: tournaments
    })
}

#[get("/tournaments/<tournament_id>/matches")]
pub fn get_matches(jwt: Claims, tournament_id: String) -> Result<JsonResponse<Vec<toornament::Match>>, CustomError> {
    let client = reqwest::Client::new();

    let matches = toornament::get_matches(&client, jwt.sub, tournament_id)?;
    
    Ok(JsonResponse {
        status: Status::Ok,
        response: matches
    })
}

#[get("/tournaments/<tournament_id>/matches/<match_id>")]
pub fn get_match(jwt: Claims, tournament_id: String, match_id: String) -> Result<JsonResponse<Vec<toornament::Game>>, CustomError> {
    let client = reqwest::Client::new();

    let toornament_match = toornament::get_games(&client, jwt.sub, tournament_id, match_id)?;
    
    Ok(JsonResponse {
        status: Status::Ok,
        response: toornament_match
    })
}

pub fn register_routes() -> Vec<Route> {
    routes![
        get_tournaments,
        get_matches,
        get_match
    ]
}

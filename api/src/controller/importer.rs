use super::super::{
    util::{jwt::*, response::*},
    service::{toornament, pubg, importer},
};

use rocket::{
    http::{Cookie, Cookies, ContentType, Status},
    Route
};
use rocket_contrib::json::Json;

#[get("/preview?<toornament_tournament_id>&<toornament_match_id>&<pubg_match_id>&<pubg_platform>")]
pub fn get_preview(jwt: Claims, toornament_tournament_id: String, toornament_match_id: String, pubg_match_id: String, pubg_platform: String) -> Result<JsonResponse<importer::Preview>, CustomError> {
    let client = reqwest::Client::new();

    let toornament_match = toornament::get_match(&client, jwt.sub, toornament_tournament_id, toornament_match_id)?;

    let pubg_match = pubg::get_match(&client, pubg_platform, pubg_match_id)?; 

    Ok(JsonResponse {
        status: Status::Ok,
        response: importer::get_preview(toornament_match, pubg_match)
    })
}

#[derive(Deserialize)]
pub struct ImportRequest {
    toornament_tournament_id: String,
    toornament_match_id: String,
    toornament_game: i64,
    pubg_platform: String,
    pubg_match_id: String
}

#[post("/import", data = "<import_request_json>")]
pub fn post_import(jwt: Claims, import_request_json: Json<ImportRequest>) -> Result<JsonResponse<toornament::Game>, CustomError> {
    let import_request = import_request_json.into_inner();
    let client = reqwest::Client::new();

    let toornament_match = toornament::get_match(&client, jwt.sub.clone(), import_request.toornament_tournament_id.clone(), import_request.toornament_match_id.clone())?;

    let pubg_match = pubg::get_match(&client, import_request.pubg_platform, import_request.pubg_match_id)?;

    let game = importer::transform_teams_to_game(toornament_match, pubg_match);

    Ok(JsonResponse {
        status: Status::Ok,
        response: toornament::patch_game(&client, jwt.sub, import_request.toornament_tournament_id, import_request.toornament_match_id, import_request.toornament_game, game)?
    })
}

pub fn register_routes() -> Vec<Route> {
    routes![
        get_preview,
        post_import
    ]
}

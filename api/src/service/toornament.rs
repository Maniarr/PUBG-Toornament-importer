use super::super::util::response::CustomError;
use super::importer;

use reqwest::{
    Client,
    header::{HeaderMap, HeaderName, HeaderValue},
    StatusCode
};
use rocket::http::Status;

use std::str::FromStr;

lazy_static!{
    pub static ref OAUTH_URI: String = std::env::var("TOORNAMENT_OAUTH_URI").unwrap().to_string();
    pub static ref API_URI: String = std::env::var("TOORNAMENT_API_URI").unwrap().to_string();
    pub static ref API_KEY: String = std::env::var("TOORNAMENT_API_KEY").unwrap().to_string();
    pub static ref CLIENT_ID: String = std::env::var("TOORNAMENT_CLIENT_ID").unwrap().to_string();
    pub static ref CLIENT_SECRET: String = std::env::var("TOORNAMENT_CLIENT_SECRET").unwrap().to_string();
    pub static ref REDIRECT_URI: String = std::env::var("REDIRECT_URI").unwrap().to_string();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub code: String,
    pub state: String
}

#[derive(Serialize, Debug)]
pub struct TokenRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    code: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub error: String,
    pub hint: String,
    pub message: String
}

impl From<LoginRequest> for TokenRequest {
    fn from(login_request: LoginRequest) -> Self {
        TokenRequest {
            grant_type: "authorization_code".to_string(),
            client_id: CLIENT_ID.clone(),
            client_secret: CLIENT_SECRET.clone(),
            redirect_uri: REDIRECT_URI.clone(),
            code: login_request.code
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: i32,
    pub token_type: String,
    pub refresh_token: String,
    pub scope: String
}


#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum ToornamentResult<T> {
    Ok(T),
    Err(Error)
}

fn unwrap_response<T>(response: ToornamentResult<T>, status: u16) -> Result<T, CustomError> {
    match response {
        ToornamentResult::Ok(objects) => Ok(objects),
        ToornamentResult::Err(error) => Err(CustomError {
            status: Status::from_code(status).unwrap_or_else(|| Status::InternalServerError),
            code: error.error,
            message: error.message
        })
    }
}

pub fn get_connection_uri(csrf_token: uuid::Uuid) -> String {
    format!(
        "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}",
        *OAUTH_URI,
        *CLIENT_ID,
        *REDIRECT_URI,
        "organizer:view organizer:result",
        csrf_token
    )
}

pub fn get_tokens(client: &Client, login_request: &LoginRequest) -> Result<TokenResponse, CustomError> {
    let mut response = client.post(&format!("{}/{}", *API_URI, "oauth/v2/token"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "grant_type=authorization_code&client_id={}&client_secret={}&redirect_uri={}&code={}",
            *CLIENT_ID,
            *CLIENT_SECRET,
            *REDIRECT_URI,
            login_request.code
        ))
        .send()?;

    match response.json::<ToornamentResult<TokenResponse>>() {
        Ok(tokens) => unwrap_response(tokens, response.status().as_u16()),
        Err(error) => {
            eprintln!("serde: {:?}", error);

            Err(CustomError::from(error))
        }
    }
}

fn get_toornament<'a>(client: &Client, url: String, range: Option<&'a str>, api_key: String) -> Result<reqwest::Response, CustomError> {
    let mut headers = HeaderMap::new();

    headers.insert(HeaderName::from_str("Authorization")?, HeaderValue::from_str(&format!("Bearer {}", api_key))?); 
    headers.insert(HeaderName::from_str("X-Api-Key")?, HeaderValue::from_str(&format!("{}", *API_KEY))?);
    headers.insert(HeaderName::from_str("Accept")?, HeaderValue::from_str("application/json")?);

    if range.is_some() {
        headers.insert(HeaderName::from_str("Range")?, HeaderValue::from_str(range.unwrap())?);
    }
    
    Ok(client.get(&format!("https://api.toornament.com/organizer/v2{}", url))
        .headers(headers)
        .send()?
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tournament {
    pub id: String,
    pub name: String,
    pub full_name: Option<String>
}

pub fn get_tournaments(client: &Client, api_key: String) -> Result<Vec<Tournament>, CustomError> {
    let mut response = get_toornament(client, format!("/tournaments?disciplines=player_unknowns_battlegrounds"), Some("tournaments=0-49"), api_key)?;
    
    unwrap_response(response.json()?, response.status().as_u16())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantField {
    pub team_id: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Participant {
    pub id: String,
    pub name: String,
    pub custom_fields: ParticipantField
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Opponent {
    pub number: i64,
    pub participant: Option<Participant>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Match {
    pub id: String,
    pub number: i64,
    pub opponents: Vec<Opponent>
}

pub fn get_matches(client: &Client, api_key: String, tournament_id: String) -> Result<Vec<Match>, CustomError> {
    let mut response = get_toornament(client, format!("/tournaments/{}/matches", tournament_id), Some("matches=0-49"), api_key)?;

    unwrap_response(response.json()?, response.status().as_u16())
}

pub fn get_match(client: &Client, api_key: String, tournament_id: String, match_id: String) -> Result<Match, CustomError> {    
    let mut response = get_toornament(client, format!("/tournaments/{}/matches/{}", tournament_id, match_id), None, api_key)?;

    unwrap_response(response.json()?, response.status().as_u16())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameOpponent {
    pub number: i64,
    pub position: i64,
    pub rank: Option<i64>,
    pub score: Option<i64>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub number: i64,
    pub status: String,
    pub opponents: Vec<GameOpponent>
}

pub fn get_games(client: &Client, api_key: String, tournament_id: String, match_id: String) -> Result<Vec<Game>, CustomError> {    
    let mut response = get_toornament(client, format!("/tournaments/{}/matches/{}/games", tournament_id, match_id), Some("games=0-49"), api_key)?;

    unwrap_response(response.json()?, response.status().as_u16()) 
}

pub fn patch_game(client: &Client, api_key: String, tournament_id: String, match_id: String, game_number:i64, game: importer::Game) -> Result<Game, CustomError> {
    let mut headers = HeaderMap::new();

    headers.insert(HeaderName::from_str("Authorization")?, HeaderValue::from_str(&format!("Bearer {}", api_key))?); 
    headers.insert(HeaderName::from_str("X-Api-Key")?, HeaderValue::from_str(&format!("{}", *API_KEY))?);
    headers.insert(HeaderName::from_str("Accept")?, HeaderValue::from_str("application/json")?);
    
    let mut response = client.patch(&format!("https://api.toornament.com/organizer/v2/tournaments/{}/matches/{}/games/{}", tournament_id, match_id, game_number))
        .headers(headers)
        .json(&game)
        .send()?;

    unwrap_response(response.json()?, response.status().as_u16())
}


use reqwest::Client;

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
    code: String,
    state: String
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
    error: String,
    hint: String,
    message: String
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

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error {
            error: "".to_string(),
            hint: "".to_string(),
            message: error.to_string()
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenResponse {
    access_token: String,
    expires_in: i32,
    token_type: String,
    refresh_token: String,
    scope: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum ToornamentResult<T> {
    Ok(T),
    Err(Error)
}

fn to_result<T>(result: ToornamentResult<T>) -> Result<T, Error> {
    match result {
        ToornamentResult::Ok(object) => Ok(object),
        ToornamentResult::Err(error) => Err(error)
    }
}

pub fn get_connection_uri() -> String {
    format!(
        "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}",
        *OAUTH_URI,
        *CLIENT_ID,
        *REDIRECT_URI,
        "organizer:view organizer:result",
        "csrf_token"
    )
}

pub fn get_tokens(client: &Client, login_request: &LoginRequest) -> Result<TokenResponse, Error> {
    let response = client.post(&format!("{}/{}", *API_URI, "oauth/v2/token"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "grant_type=authorization_code&client_id={}&client_secret={}&redirect_uri={}&code={}",
            *CLIENT_ID,
            *CLIENT_SECRET,
            *REDIRECT_URI,
            login_request.code
        ))
        .send()?
        .json::<ToornamentResult<TokenResponse>>()?;

    to_result(response)
}

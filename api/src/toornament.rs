use reqwest::Client;

#[derive(Debug)]
pub struct ToornamentApi {
    pub api_uri: String,
    pub api_key: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokenResponse {
    TokenResult {
        scope: String,
        token_type: String,
        expires_in: u64,
        access_token: String,
        refresh_token: String
    },
    TokenError {
        error: String,
        message: String,
        hint: Option<String>
    }
}

pub trait ToornamentAuthorizer {
    fn authorize_with_code(&mut self, code: String) -> Result<TokenResponse, String>;
}

impl ToornamentAuthorizer for ToornamentApi {
    fn authorize_with_code(&mut self, code: String) -> Result<TokenResponse, String> {
        let client = Client::new();

        let params = [
            ("grant_type", "authorization_code"),
            ("client_id", &format!("{}", self.client_id)),
            ("client_secret", &format!("{}", self.client_secret)),
            ("redirect_uri", &format!("{}", self.redirect_uri)),
            ("code", code.as_str())
        ];

        let res = client.post(&format!("{}{}", self.api_uri, "/oauth/v2/token"))
            .form(&params)
            .send();

        if res.is_err() {
            println!("{:?}", res);

            return Err("ko".to_string());
        }

        match res.unwrap().json::<TokenResponse>() {
            Ok(tokens)  => Ok(tokens),
            Err(e)      => Err(e.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;
    use std::fs::File;
    use std::io::Read;

    use serde_json::*;

    #[test]
    fn internal() {
        let mut file = File::open("fixtures/token_ok.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let data: TokenResponse = serde_json::from_str(&contents).unwrap();
        println!("{:#?}", data);
    }
}

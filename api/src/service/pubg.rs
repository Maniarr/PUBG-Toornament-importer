use super::super::util::response::CustomError;

use reqwest::Client;

lazy_static!{
    pub static ref API_KEY: String = std::env::var("PUBG_API_KEY").unwrap().to_string();
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
   pub created_at: String 
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tournament {
    #[serde(rename = "type")]
    pub type_entity: String,
    pub id: String,
    pub attributes: Attribute
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PubgResponse<T> {
    pub data: T,
    pub included: Option<Vec<RelationshipItem>>
}

fn get_pubg(client: &Client, url: String) -> Result<reqwest::Response, CustomError> {
    Ok(client.get(&format!("https://api.pubg.com{}", url))
        .header("Authorization", format!("Bearer {}", *API_KEY))
        .header("Accept", "application/vnd.api+json")
        .send()?)
}

pub fn get_tournaments(client: &Client) -> Result<PubgResponse<Vec<Tournament>>, CustomError> {
    let mut response = get_pubg(client, "/tournaments".to_string())?;

    Ok(response.json::<PubgResponse<Vec<Tournament>>>()?)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RelationshipItem {
    #[serde(rename = "type")]
    pub entity_type: String,
    pub id: String,
    pub attributes: Option<Attribute>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Relationship {
    pub matches: PubgResponse<Vec<RelationshipItem>>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TournamentInfo {
    #[serde(rename = "type")]
    pub entity_type: String,
    pub id: String,
    pub relationships: Relationship,
}

pub fn get_tournament(client: &Client, tournament_id: String) -> Result<PubgResponse<TournamentInfo>, CustomError> {
    let mut response = get_pubg(client, format!("/tournaments/{}", tournament_id))?;

    let test = response.text()?.clone();

    Ok(serde_json::from_str::<PubgResponse<TournamentInfo>>(&test)?)
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchAttribute {
    pub map_name: String,
    pub is_custom_match: bool,
    pub season_state: String,
    pub created_at: String,
    pub duration: i64,
    pub title_id: String,
    // tags
    // stats
    pub game_mode: String,
    pub shard_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MatchRelationship {
    pub rosters: PubgResponse<Vec<RelationshipItem>>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParticipantStat {
    pub kills: i64
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantAttribute {
    pub stats: ParticipantStat,
    pub actor: String,
    pub shard_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParticipantInclude {
    pub id: String,
    pub attributes: ParticipantAttribute
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RosterStat {
    pub rank: i64,
    pub team_id: i64
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RosterAttribute {
    pub stats: RosterStat,
    pub won: String,
    pub shard_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RosterRelationship {
    pub team: PubgResponse<Option<RelationshipItem>>,
    pub participants: PubgResponse<Vec<RelationshipItem>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RosterInclude {
    pub id: String,
    pub attributes: RosterAttribute,
    pub relationships: RosterRelationship
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Unknown {
    #[serde(rename = "type")]
    pub entity_type: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Include {
    Participant(ParticipantInclude),
    Roster(RosterInclude),
    #[serde(other)]
    Unknown
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Match {
    #[serde(rename = "type")]
    pub entity_type: String,
    pub id: String,
    pub attributes: MatchAttribute,
    pub relationships: MatchRelationship,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MatchResponse {
    pub data: Match,
    pub included: Vec<Include>
}

pub fn get_match(client: &Client, region: String, match_id: String) -> Result<MatchResponse, CustomError> {
    let mut response = get_pubg(client, format!("/shards/{}/matches/{}", region, match_id))?;
 
    Ok(response.json::<MatchResponse>()?)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Error {
    pub title: String,
    pub detail: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Errors {
    pub errors: Vec<Error>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    Ok(T),
    Err(Errors)
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAttribute {
    pub shard_id: String,
    pub name: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlayerRelationship {
    pub matches: PubgResponse<Vec<RelationshipItem>>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Player {
    pub id: String,
    pub attributes: PlayerAttribute,
    pub relationships: PlayerRelationship
}

pub fn get_players(client: &Client, username: String, platform: String) -> Result<PubgResponse<Vec<Player>>, CustomError> {
    let mut response = get_pubg(client, format!("/shards/{}/players?filter[playerNames]={}", platform, username))?;

    Ok(response.json::<PubgResponse<Vec<Player>>>()?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_include_participant() {
        let json = r#"
        {
            "type": "participant",
            "id": "849de2f9-0ac4-4a6c-b133-91e80cd70e97",
            "attributes": {
                "stats": {
                    "DBNOs": 0,
                    "assists": 0,
                    "boosts": 2,
                    "damageDealt": 0,
                    "deathType": "alive",
                    "headshotKills": 0,
                    "heals": 4,
                    "killPlace": 2,
                    "killPoints": 0,
                    "killPointsDelta": 0,
                    "killStreaks": 0,
                    "kills": 0,
                    "lastKillPoints": 0,
                    "lastWinPoints": 0,
                    "longestKill": 0,
                    "mostDamage": 0,
                    "name": "KG_970",
                    "playerId": "account.2d334e7f1f7846ec8b76bb8c21c86d7f",
                    "rankPoints": 0,
                    "revives": 0,
                    "rideDistance": 0,
                    "roadKills": 0,
                    "swimDistance": 0,
                    "teamKills": 0,
                    "timeSurvived": 858.274,
                    "vehicleDestroys": 0,
                    "walkDistance": 1722.19861,
                    "weaponsAcquired": 20,
                    "winPlace": 1,
                    "winPoints": 0,
                    "winPointsDelta": 0
                },
                "actor": "",
                "shardId": "tournament"
            }
        }
        "#;

        println!("{:?}", serde_json::from_str::<Include>(json).unwrap());
    }
}

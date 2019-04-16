use super::{
    pubg,
    toornament
};

use std::collections::HashMap;

#[derive(Serialize, Debug)]
struct Team {
    team_id: i64,
    rank: i64,
    kills: i64
}

#[derive(Serialize, Debug, Clone)]
pub struct ConversionTeam {
    pub name: String,
    pub number: i64
}

fn get_opponents_conversion(toornament_match: &toornament::Match) -> HashMap<String, ConversionTeam> {
    let mut conversions = HashMap::new();

    for opponent in &toornament_match.opponents {
        if let Some(participant) = &opponent.participant {
            if let Some(team_id) = &participant.custom_fields.team_id {
                conversions.insert(
                    team_id.clone(),
                    ConversionTeam {
                        name: participant.name.clone(),
                        number: opponent.number.clone()
                    }
                );
            }
        }
    }

    conversions
}

fn get_teams(pubg_match: &pubg::MatchResponse) -> Vec<Team> {
    let mut teams = Vec::new();

    for include in &pubg_match.included {
        if let pubg::Include::Roster(roster) = include {
            let mut team = Team {
                team_id: roster.attributes.stats.team_id.clone(),
                rank: roster.attributes.stats.rank.clone(),
                kills: 0
            };

            for participant in &roster.relationships.participants.data {
                for item in &pubg_match.included {
                    if let pubg::Include::Participant(participant_item) = item {
                        if participant.id == participant_item.id {
                            team.kills += participant_item.attributes.stats.kills;
                        }
                    }
                }
            }

            teams.push(team);
        }
    }
    
    teams
}

#[derive(Serialize, Debug)]
pub struct PreviewProperty {
    pub ingame_rank: Option<i64>,
    pub kills: Option<i64>
}

#[derive(Serialize, Debug)]
pub struct PreviewTeam {
    pub team: ConversionTeam,
    pub properties: PreviewProperty 
}

#[derive(Serialize, Debug)]
pub struct Preview {
    pub teams: Vec<PreviewTeam>
}

pub fn get_preview(toornament_match: toornament::Match, pubg_match: pubg::MatchResponse) -> Preview {
    let teams = get_teams(&pubg_match);
    let conversions = get_opponents_conversion(&toornament_match);
    let mut preview_teams: Vec<PreviewTeam> = Vec::new();

    for team in teams {
        if let Some(conversion) = conversions.get(&team.team_id.to_string()) {
            preview_teams.push(PreviewTeam {
                team: conversion.clone(),
                properties: PreviewProperty {
                    ingame_rank: Some(team.rank),
                    kills: Some(team.kills)
                }
            }); 
        }
    }

    Preview {
        teams: preview_teams
    }
}

#[derive(Serialize)]
pub struct GameOpponent {
    pub number: i64,
    pub properties: PreviewProperty
}

#[derive(Serialize)]
pub struct Game {
    pub status: String,
    pub opponents: Vec<GameOpponent>
}

pub fn transform_teams_to_game(toornament_match: toornament::Match, pubg_match: pubg::MatchResponse) -> Game {
    let teams = get_teams(&pubg_match);
    let conversions = get_opponents_conversion(&toornament_match);
    let mut opponents: Vec<GameOpponent> = Vec::new();

    for team in teams {
        if let Some(conversion) = conversions.get(&team.team_id.to_string()) {
            opponents.push(GameOpponent {
                number: conversion.number.clone(),
                properties: PreviewProperty {
                    ingame_rank: Some(team.rank),
                    kills: Some(team.kills)
                }
            }); 
        }
    }

    for team in toornament_match.opponents {
        let mut finded = false;

        for opponent in &opponents {
            if opponent.number == team.number {
                finded = true;

                break;
            }
        }

        if finded == false {
            opponents.push(GameOpponent {
                number: team.number.clone(),
                properties: PreviewProperty {
                    ingame_rank: None,
                    kills: None
                }
            });
        }
    }

    Game {
        status: "completed".to_string(),
        opponents
    }
}


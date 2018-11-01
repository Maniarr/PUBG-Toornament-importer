import json

class Team:
    def __init__(self):
        self.team_id = None
        self.rank = None
        self.kills = 0

def get_teams(match):
    teams = []

    for item in match["included"]:
        if item["type"] != "roster":
            continue

        team = Team()

        team.rank = item["attributes"]["stats"]["rank"]
        team.team_id = item["attributes"]["stats"]["teamId"]

        for participant in item["relationships"]["participants"]["data"]:
            for participant_item in match["included"]:
                if participant["id"] != participant_item["id"]:
                    continue

                team.kills = team.kills + participant_item["attributes"]["stats"]["kills"]
        
        teams.append(team)

    return teams

def get_opponents_conversion(toornament_match):
    conversion = {}

    for opponents in toornament_match["opponents"]:
        conversion[opponents["participant"]["custom_fields"]["team_id"]] = opponents["number"]

    return conversion

def transform_teams_to_games(teams, toornament_match):
    game = {
        "status": "completed",
        "opponents": []
    }

    conversion = get_opponents_conversion(toornament_match)

    for team in teams:
        game["opponents"].append({
            "number": conversion[str(team.team_id)],
            "properties": {
                "ingame_rank": team.rank,
                "kills": team.kills
            }
        })

    return game

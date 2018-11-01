import argparse
import pubg
import toornament
import importer

def parse_args():
    parser = argparse.ArgumentParser(description = "Import PUBG match to Toornament")

    parser.add_argument("--pubg-api-key", dest="pugb_api_key")
    
    parser.add_argument("--pubg-region", dest="pubg_region")
    parser.add_argument("--pubg-match-id", dest="pubg_match_id")

    parser.add_argument("--toornament-api-key", dest="toornament_api_key")
    parser.add_argument("--toornament-client-id", dest="toornament_client_id")
    parser.add_argument("--toornament-client-secret", dest="toornament_client_secret")
    parser.add_argument("--toornament-token", dest="toornament_token")

    parser.add_argument("--toornament-tournament-id", dest="toornament_tournament_id")
    parser.add_argument("--toornament-match-id", dest="toornament_match_id")
    parser.add_argument("--toornament-game", dest="toornament_game")
    
    return parser.parse_args()

def main():
    args = parse_args()

    pubg_api = pubg.Api(args.pugb_api_key)

    toornament_api = toornament.Api(
        args.toornament_api_key,
        args.toornament_client_id,
        args.toornament_client_secret
    )

    print("Retrieve token for Toornament api")
    toornament_api.get_token("organizer:view organizer:result")

    print("Retrieve match from PUBG api")
    pubg_match = pubg_api.get_match(args.pubg_region, args.pubg_match_id)

    teams = importer.get_teams(pubg_match)
 
    print("Retrieve match from Toornament api")
    toornament_match = toornament_api.get_match(
        args.toornament_tournament_id,
        args.toornament_match_id
    )

    game = importer.transform_teams_to_games(teams, toornament_match)

    print("Update game result on Toornament")
    toornament_api.patch_toornament_game(
        args.toornament_tournament_id,
        args.toornament_match_id,
        args.toornament_game,
        game
    )

if __name__ == "__main__":
    main()

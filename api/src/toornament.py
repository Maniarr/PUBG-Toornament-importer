import requests
import json

class ApiUser:
    def __init__(self, api):
        self.api = api
        self.access_token = None

    def get_tokens(self, code):
        r = requests.post("https://api.toornament.com/oauth/v2/token", data = {
            "grant_type": "authorization_code",
            "client_id": self.api.client_id,
            "client_secret": self.api.client_secret,
            "redirect_uri": self.api.redirect_uri,
            "code": code
        })

        if r.status_code != 200:
            return None

        return r.json()

    def get_tournaments(self):
        r = requests.get("https://api.toornament.com/organizer/v2/tournaments?disciplines=player_unknowns_battlegrounds", headers = {
            "X-Api-Key": self.api.api_key,
            "Authorization": self.access_token,
            "Range": "tournaments=0-19"
        })

        print(r)
        print(r.text)

        return r.json()
    
    def get_matches(self, tournament_id):
        r = requests.get("https://api.toornament.com/organizer/v2/tournaments/{}/matches".format(tournament_id), headers = {
            "X-Api-Key": self.api.api_key,
            "Authorization": self.access_token,
            "Range": "matches=0-49"
        })

        print(r)
        print(r.text)

        return r.json()

    def get_games(self, tournament_id, match_id):
        r = requests.get("https://api.toornament.com/organizer/v2/tournaments/{}/matches/{}/games".format(tournament_id, match_id), headers = {
            "X-Api-Key": self.api.api_key,
            "Authorization": self.access_token,
            "Range": "games=0-49"
        })

        print(r)
        print(r.text)

        return r.json()

    def get_match(self, tournament_id, match_id):
        r = requests.get("https://api.toornament.com/organizer/v2/tournaments/{}/matches/{}".format(tournament_id, match_id), headers = {
            "X-Api-Key": self.api.api_key,
            "Authorization": self.access_token
        })

        print(r)

        return r.json()

    def patch_toornament_game(self, tournament_id, match_id, game_number, data):
        r = requests.patch("https://api.toornament.com/organizer/v2/tournaments/{}/matches/{}/games/{}".format(tournament_id, match_id, game_number), headers = {
            "X-Api-Key": self.api.api_key,
            "Authorization": self.access_token
        }, data=json.dumps(data))
        
        print(r)

class Api:
    def __init__(self, api_key, client_id, client_secret, redirect_uri):
        self.api_key = api_key
        self.client_id = client_id
        self.client_secret = client_secret
        self.redirect_uri = redirect_uri
        self.token = None

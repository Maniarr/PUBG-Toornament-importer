import requests
import json

class Api:
    def __init__(self, api_key, client_id, client_secret):
        self.api_key = api_key
        self.client_id = client_id
        self.client_secret = client_secret
        self.token = None

    def get_token(self, scope):
        r = requests.post("https://api.toornament.com/oauth/v2/token", headers = {
            "x-api-key": self.api_key
        }, data = {
            "grant_type": "client_credentials",
            "client_id": self.client_id,
            "client_secret": self.client_secret,
            "scope": scope
        })

        print(r)

        response = r.json()

        self.token = response["access_token"]

    def get_match(self, tournament_id, match_id):
        r = requests.get("https://api.toornament.com/organizer/v2/tournaments/{}/matches/{}".format(tournament_id, match_id), headers = {
            "X-Api-Key": self.api_key,
            "Authorization": self.token
        })

        print(r)

        return r.json()

    def patch_toornament_game(self, tournament_id, match_id, game_number, data):
        r = requests.patch("https://api.toornament.com/organizer/v2/tournaments/{}/matches/{}/games/{}".format(tournament_id, match_id, game_number), headers = {
            "X-Api-Key": self.api_key,
            "Authorization": self.token
        }, data=json.dumps(data))
        
        print(r)
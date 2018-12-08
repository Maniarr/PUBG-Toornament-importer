import requests

class Api:
    def __init__(self, api_key):
        self.api_key = api_key

    def get_tournaments(self):
        r = requests.get("https://api.pubg.com/tournaments", headers={
            "Authorization": "Bearer {}".format(self.api_key),
            "Accept": "application/vnd.api+json"
        })

        print(r)

        return r.json()

    def get_tournament(self, id):
        r = requests.get("https://api.pubg.com/tournaments/{}".format(id), headers={
            "Authorization": "Bearer {}".format(self.api_key),
            "Accept": "application/vnd.api+json"
        })

        print(r)

        return r.json()

    def get_match(self, region, id):
        r = requests.get("https://api.pubg.com/shards/{}/matches/{}".format(region, id), headers={
            "Authorization": "Bearer {}".format(self.api_key),
            "Accept": "application/vnd.api+json"
        })

        print(r)

        return r.json()

    def search_players(self, region, username):
        r = requests.get("https://api.pubg.com/shards/{}/players".format(region), headers={
            "Authorization": "Bearer {}".format(self.api_key),
            "Accept": "application/vnd.api+json"
        }, params={
            "filter[playerNames]": username
        })

        print(r)

        return r.json()

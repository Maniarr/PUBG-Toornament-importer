import requests

class Api:
    def __init__(self, api_key):
        self.api_key = api_key

    def get_match(self, region, id):
        r = requests.get("https://api.pubg.com/shards/{}/matches/{}".format(region, id), headers={
            "Authorization": "Bearer {}".format(self.api_key),
            "Accept": "application/vnd.api+json"
        })

        print(r)

        return r.json()
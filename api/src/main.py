import os
import uuid
import json

import toornament
import pubg
import importer

from flask import Flask, session, request, redirect
from flask_cors import CORS

app = Flask(__name__)
CORS(app, supports_credentials=True)
SESSION_TYPE = "filesystem"
SESSION_FILE_DIR = "./session"
app.secret_key = os.getenv("SECRET_KEY")
app.config.from_object(__name__)

toornament_api = toornament.Api(
    os.getenv("TOORNAMENT_API_KEY"),
    os.getenv("TOORNAMENT_CLIENT_ID"),
    os.getenv("TOORNAMENT_CLIENT_SECRET"),
    os.getenv("REDIRECT_URI")
)

pubg_api = pubg.Api(
    os.getenv("PUBG_API_KEY")
)

def create_json_response(data, status_code):
    return app.response_class(
        response=json.dumps(data),
        status=status_code,
        mimetype='application/json'
    )

@app.route("/login", methods=["GET"])
def home():
    csrf_token = str(uuid.uuid4())

    session["csrf_token"] = csrf_token

    return create_json_response({
        "connection_uri": "https://account.toornament.com/oauth2/authorize?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}".format(
            os.getenv("TOORNAMENT_CLIENT_ID"),
            os.getenv("REDIRECT_URI"),
            "organizer:view organizer:result",
            csrf_token
        )
    }, 200)

@app.route("/login", methods=["POST"])
def login():
    if "code" not in request.json:
        return create_json_response({
            "error": "No code provided"
        }, 401)

    if "state" not in request.json:
        return create_json_response({
            "error": "No state provided"
        }, 401)

    if "csrf_token" not in session:
        return create_json_response({
            "error": "No CSRF provided"
        }, 401)

    code = request.json["code"]
    state = request.json["state"]

    if state != session["csrf_token"]:
        return  create_json_response({
            "error": "State/CSRF token not valid"
        }, 401)

    session.pop("csrf_token")

    api_user = toornament.ApiUser(toornament_api)

    token = api_user.get_tokens(code)

    if token is None:
        return create_json_response({
            "error": "retry"
        }, 401)

    return create_json_response(token, 200)

@app.route("/tournaments", methods=["GET"])
def tournaments():
    if "Authorization" not in request.headers:
        return create_json_response({
            "error": "No \"Authorization\" header provided"
        }, 401)

    api_user = toornament.ApiUser(toornament_api)
    api_user.access_token = request.headers["Authorization"]

    return create_json_response(api_user.get_tournaments(), 200)

@app.route("/tournaments/<tournament_id>/matches", methods=["GET"])
def tournament(tournament_id):
    if "Authorization" not in request.headers:
        return create_json_response({
            "error": "No \"Authorization\" header provided"
        }, 401)

    api_user = toornament.ApiUser(toornament_api)
    api_user.access_token = request.headers["Authorization"]

    return create_json_response(api_user.get_matches(tournament_id), 200)

@app.route("/tournaments/<tournament_id>/matches/<match_id>", methods=["GET"])
def games(tournament_id, match_id):
    if "Authorization" not in request.headers:
        return create_json_response({
            "error": "No \"Authorization\" header provided"
        }, 401)

    api_user = toornament.ApiUser(toornament_api)
    api_user.access_token = request.headers["Authorization"]

    return create_json_response(api_user.get_games(tournament_id, match_id), 200)

@app.route("/pubg/tournaments", methods=["GET"])
def pubg_tournaments():
    if "Authorization" not in request.headers:
        return create_json_response({
            "error": "No \"Authorization\" header provided"
        }, 401)

    return create_json_response(pubg_api.get_tournaments(), 200)

@app.route("/pubg/tournaments/<tournament_id>", methods=["GET"])
def pubg_tournament(tournament_id):
    if "Authorization" not in request.headers:
        return create_json_response({
            "error": "No \"Authorization\" header provided"
        }, 401)

    return create_json_response(pubg_api.get_tournament(tournament_id), 200)

@app.route("/import", methods=["POST"])
def import_statistics():
    if "Authorization" not in request.headers:
        return create_json_response({
            "error": "No \"Authorization\" header provided"
        }, 401)

    api_user = toornament.ApiUser(toornament_api)
    api_user.access_token = request.headers["Authorization"]

    print({
        "toornament_tournament_id": request.json["toornament_tournament_id"],
        "toornament_match_id": request.json["toornament_match_id"],
        "toornament_game": request.json["toornament_game"], 
        "pubg_match_id": request.json["pubg_match_id"]})

    pubg_match = pubg_api.get_match("pc-tournament", request.json["pubg_match_id"])

    teams = importer.get_teams(pubg_match)

    print(len(teams))
 
    print("Retrieve match from Toornament api")

    toornament_match = api_user.get_match(
        request.json["toornament_tournament_id"],
        request.json["toornament_match_id"]
    )

    game = importer.transform_teams_to_games(teams, toornament_match)

    print("Update game result on Toornament")
    api_user.patch_toornament_game(
        request.json["toornament_tournament_id"],
        request.json["toornament_match_id"],
        request.json["toornament_game"],
        game
    )

    return create_json_response({}, 200)

if __name__ == "__main__":
    app.debug = True
    app.run(host="127.0.0.1")

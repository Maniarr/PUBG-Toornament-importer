import os
import uuid
import json

import toornament
import pubg

from flask import Flask, session, request, redirect

app = Flask(__name__)
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

@app.route("/")
def home():
    if "toornament_token" in session:
        return "access: {}".format(session["toornament_token"]["access_token"])

    csrf_token = str(uuid.uuid4())

    session["csrf_token"] = csrf_token

    return "<a href=\"https://account.toornament.com/oauth2/authorize?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}\">Login with Toornament</a>".format(
        os.getenv("TOORNAMENT_CLIENT_ID"),
        os.getenv("REDIRECT_URI"),
        "organizer:view organizer:result",
        csrf_token
    )

@app.route("/login")
def login():
    code = request.args.get("code") 
    print(code)

    if code is None:
        return "No code provided by Toornament"
    
    if "csrf_token" not in session:
        return redirect("/", code=302)

    if request.args.get("state") != session["csrf_token"]:
        return "CSRF token not valid"

    session.pop("csrf_token")

    api_user = toornament.ApiUser(toornament_api)

    token = api_user.get_tokens(code)

    if token is not None:
        session["toornament_token"] = token

    return redirect("/", code=302)

@app.route("/tournaments")
def tournaments():
    if "toornament_token" not in session:
        return redirect("/", code=302)

    api_user = toornament.ApiUser(toornament_api)
    api_user.access_token = session["toornament_token"]["access_token"]

    return json.dumps(api_user.get_tournaments())


@app.route("/tournaments/<tournament_id>/matches")
def tournament(tournament_id):
    if "toornament_token" not in session:
        return redirect("/", code=302)

    api_user = toornament.ApiUser(toornament_api)
    api_user.access_token = session["toornament_token"]["access_token"]

    return json.dumps(api_user.get_matches(tournament_id))

@app.route("/tournaments/<tournament_id>/matches/<match_id>")
def games(tournament_id, match_id):
    if "toornament_token" not in session:
        return redirect("/", code=302)

    api_user = toornament.ApiUser(toornament_api)
    api_user.access_token = session["toornament_token"]["access_token"]

    return json.dumps(api_user.get_games(tournament_id, match_id))

@app.route("/pubg/tournaments")
def pubg_tournaments():
    if "toornament_token" not in session:
        return redirect("/", code=302)

    print(pubg_api.api_key)

    return json.dumps(pubg_api.get_tournaments())

@app.route("/pubg/tournaments/<tournament_id>")
def pubg_tournament(tournament_id):
    if "toornament_token" not in session:
        return redirect("/", code=302)

    print(pubg_api.api_key)

    return json.dumps(pubg_api.get_tournament(tournament_id))

if __name__ == "__main__":
    app.debug = True
    app.run(host="127.0.0.1")

<template>
    <div>
        <nav class="navbar navbar-light bg-light justify-content-between">
            <a class="navbar-brand" href="#">PUBG importer</a>
            <button class="btn btn-danger" @click="logout">Logout</button>
        </nav>

        <div class="container mt-4">
            <div class="row">
                <div v-if="tournament_id == null" class="col">
                    <h4>Choose Toornament tournament:</h4>

                    <div class="container pt-3 pb-3">
                        <div class="card-columns">
                            <div v-for="tournament in tournaments" v-bind:key="tournament.id" @click="setTournament(tournament.id)" class="card" style="cursor: pointer">
                                <div class="card-body">
                                    <h5 class="card-title">{{ tournament.name }}</h5>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div v-if="tournament_id != null && game == null" class="col">
                    <h4>Choose Toornament game:</h4>

                    <div class="container pt-3 pb-3">
                        <div class="card-columns">
                            <div v-for="game in games" v-bind:key="game.number" @click="setGame(game.number)" class="card" style="cursor: pointer">
                                <div class="card-body">
                                    <h5 class="card-title">Game #{{ game.number }}</h5>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div v-if="tournament_id != null && game != null && pubg_tournament_id == null" class="col">
                    <h4>Choose PUBG tournament:</h4>

                    <div class="container pt-3 pb-3">
                        <div class="card-columns">
                            <div v-for="tournament in pubg_tournaments" v-bind:key="tournament.id" @click="setPUBGTournament(tournament.id)" class="card"
                                style="cursor: pointer">
                                <div class="card-body">
                                    <h5 class="card-title">{{ tournament.id }}</h5>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div v-if="tournament_id != null && game != null && pubg_tournament_id != null && pubg_match_id == null" class="col">
                    <h4>Choose PUBG match:</h4>

                    <div class="container pt-3 pb-3">
                        <div v-for="match in pubg_matches" v-bind:key="match.id" @click="setPUBGMatch(match.id)" class="card mb-2" style="cursor: pointer; width: 100%">
                            <div class="card-body">
                                <h5 class="card-title">Played at {{ match.attributes.createdAt }} - {{ match.id }}</h5>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
    import Config from '../config'

    export default {
        name: 'Tournament',
        props: {
            accesstoken: String
        },
        data: function () {
            return {
                tournament_id: null,
                match_id: null,
                game: null,
                pubg_tournament_id: null,
                pubg_match_id: null,
                tournaments: [],
                matches: [],
                games: [],
                pubg_tournaments: [],
                pubg_matches: []
            }
        },
        methods: {
            logout: function () {
                window.location = '/'
            },
            getTournaments: function () {
                this.$http.get(Config.API_URL + '/tournaments', {
                    headers: {
                        'Authorization': 'Bearer ' + this.accesstoken
                    }
                }).then(response => {
                    this.$data.tournaments = response.body
                })
            },
            getMatches: function (tournament_id) {
                return this.$http.get(Config.API_URL + '/tournaments/' + tournament_id + '/matches', {
                    headers: {
                        'Authorization': 'Bearer ' + this.accesstoken
                    }
                }).then(response => {
                    return this.$data.matches = response.body
                })
            },
            getGames: function (tournament_id, match_id) {
                this.$http.get(Config.API_URL + '/tournaments/' + tournament_id + '/matches/' + match_id, {
                    headers: {
                        'Authorization': 'Bearer ' + this.accesstoken
                    }
                }).then(response => {
                    this.$data.games = response.body
                })
            },
            setTournament: function (tournament_id) {
                this.getMatches(tournament_id).then(matches => {
                    this.$data.tournament_id = tournament_id

                    this.$data.match_id = this.$data.matches[0].id
                    this.getGames(tournament_id, this.$data.match_id)
                });
            },
            setGame: function (game) {
                this.$data.game = game
                this.getPUBGTournaments()
            },
            getPUBGTournaments: function () {
                this.$http.get(Config.API_URL + '/pubg/tournaments', {
                    headers: {
                        'Authorization': 'Bearer ' + this.accesstoken
                    }
                }).then(response => {
                    this.$data.pubg_tournaments = response.body.data
                })
            },
            getPUBGMatches: function (tournament_id) {
                this.$http.get(Config.API_URL + '/pubg/tournaments/' + tournament_id, {
                    headers: {
                        'Authorization': 'Bearer ' + this.accesstoken
                    }
                }).then(response => {
                    this.$data.pubg_matches = response.body.included.sort((a, b) => {
                        return a.attributes.createdAt < b.attributes.createdAt ? 1 : -1
                    })
                })
            },
            setPUBGTournament: function (tournament_id) {
                this.$data.pubg_tournament_id = tournament_id
                this.getPUBGMatches(tournament_id)
            },
            setPUBGMatch: function (match_id) {
                this.$data.pubg_match_id = match_id

                this.$http.post(Config.API_URL + '/import', {
                    toornament_tournament_id: this.$data.tournament_id,
                    toornament_match_id: this.$data.match_id,
                    toornament_game: this.$data.game,
                    pubg_match_id: this.$data.pubg_match_id,
                },
                    {
                        headers: {
                            'Authorization': 'Bearer ' + this.accesstoken
                        }
                    }).then(() => {
                        this.$data.tournament_id = null
                        this.$data.match_id = null
                        this.$data.game = null
                        this.$data.pubg_tournament_id = null
                        this.$data.pubg_match_id = null
                        this.$data.tournaments = []
                        this.$data.matches = []
                        this.$data.games = []
                        this.$data.pubg_tournaments = []
                        this.$data.pubg_matches = []

                        this.getTournaments()
                    })
            },
        },
        mounted: function () {
            this.getTournaments()
        }
    }
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
<template>
    <div>
        <div class="row">
            <div v-if="tournament == null" class="col">
                <h4>Choose Toornament tournament:</h4>

                <div class="row pt-3 pb-3">
                    <div class="card-columns">
                        <div v-for="tournament in tournaments" v-bind:key="tournament.id" @click="setTournament(tournament)" class="card" style="cursor: pointer">
                            <div class="card-body">
                                <h5 class="card-title">{{ tournament.name }}</h5>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            
            <div v-if="tournament != null && game == null" class="col">
                <h4>Choose Toornament game:</h4>

                <div class="row pt-3 pb-3">
                    <div class="card-columns">
                        <div v-for="game in games" v-bind:key="game.number" @click="setGame(game.number)" class="card" style="cursor: pointer">
                            <div class="card-body">
                                <h5 class="card-title">Game #{{ game.number }}</h5>
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
        name: 'ToornamentChooser',
        props: {
            accesstoken: String
        },
        data: function () {
            return {
                tournament: null,
                match: null,
                game: null,
                tournaments: [],
                matches: [],
                games: []
            }
        },
        methods: {
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
            setTournament: function (tournament) {
                this.getMatches(tournament.id).then(matches => {
                    this.$data.tournament = tournament
                    this.udpate_toornament()

                    this.$data.match = this.$data.matches[0]
                    this.getGames(tournament.id, this.$data.match.id)
                });
            },
            setGame: function (game) {
                this.$data.game = game
                this.udpate_toornament()
            },
            udpate_toornament: function() {
                this.$emit('update_toornament', {
                    tournament: this.$data.tournament,
                    game: this.$data.game
                })
            }
        },
        mounted: function () {
            this.udpate_toornament()
            this.getTournaments()
        }
    }
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
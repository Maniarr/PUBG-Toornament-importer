<template>
    <div>
        <div class="row">
            <div v-if="tournament == null" class="col">
                <h4>Choose Toornament tournament:</h4>

                <div class="row pt-3 pb-3">
                    <div class="card-columns">
                        <div v-for="tournament in tournaments" v-bind:key="tournament.id" @click="set_tournament(tournament)" class="card" style="cursor: pointer">
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
                        <div v-for="game in games" v-bind:key="game.number" @click="set_game(game.number)" class="card" style="cursor: pointer">
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
            get_tournaments: function () {
               this.$api.toornament_get_tournaments().then(tournaments => {
                    this.$data.tournaments = tournaments
                })
            },
            set_tournament: function (tournament) {
                this.$api.toornament_get_matches(tournament.id).then(matches => {
                    this.$data.tournament = tournament
                    this.udpate_toornament()

                    this.$data.match = matches[0]
                    this.$api.toornament_get_games(tournament.id, this.$data.match.id).then(games => {
                        this.$data.games = games
                    })
                });
            },
            set_game: function (game) {
                this.$data.game = game
                this.udpate_toornament()
            },
            udpate_toornament: function() {
                this.$emit('update_toornament', {
                    tournament: this.$data.tournament,
                    match: this.$data.match,
                    game: this.$data.game
                })
            }
        },
        mounted: function () {
            this.get_tournaments()
        }
    }
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
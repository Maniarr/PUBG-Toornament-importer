<template>                
    <div class="row">
        <div class="col col-12">
            <ul class="nav nav-tabs">
                <li class="nav-item">
                    <a class="nav-link" @click="mode = 'custom_match'; selected_platform = 'steam'" v-bind:class="{ active: mode == 'custom_match' }" href="#">Custom match</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" @click="mode = 'tournament'; selected_platform = 'pc-tournament'" v-bind:class="{ active: mode == 'tournament' }" href="#">Tournament</a>
                </li>
            </ul>
        </div>

        <div v-if="mode == 'custom_match'" class="col col-12 mt-3 mb-3">
            <div class="row">
                <div class="col col-4">
                    <div class="input-group mb-3">
                        <div class="input-group-prepend">
                            <label class="input-group-text">Platform</label>
                        </div>
                        <select class="custom-select" v-model="selected_platform">
                            <option v-for="platform in platforms" v-bind:key="platform" v-bind:value="platform">{{ platform }}</option>
                        </select>
                    </div>
                </div>
                <div class="col col-8">
                    <div class="input-group">
                        <input v-model="username" type="text" class="form-control" placeholder="Username" aria-label="Recipient's username" aria-describedby="button-addon2">
                        <div class="input-group-append">
                            <button @click="search_player" class="btn btn-success" type="button">Search</button>
                        </div>
                    </div>
                </div>
            </div>
            <div class="row">
                <div v-if="errors != null" class="col col-12">
                    <div class="alert alert-danger">
                        <p v-for="(error, index) in errors" v-bind:key="index">{{ error.detail }}</p>
                    </div>  
                </div>
                <div v-else>
                    <div v-if="matches == null" class="card-columns">
                        <div v-for="player in players" v-bind:key="player.id" @click="get_matches_by_player(player)" class="card" style="cursor: pointer">
                            <div class="card-body">
                                <h5 class="card-title">{{ player.attributes.name }}</h5>
                            </div>
                        </div>
                    </div>
                    <div v-else class="row">
                        <div class="col-12" v-for="match in matches" v-bind:key="match.id">
                            <PUBGMatch
                                :id = "match.id"
                                :platform = "selected_platform"
                                @click.native="update_match(match.id)"
                            >
                            </PUBGMatch>   
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div v-if="mode == 'tournament'">
            <div v-if="tournament == null" class="col">
                <h4>Choose PUBG tournament:</h4>

                <div class="row pt-3 pb-3">
                    <div class="card-columns">
                        <div v-for="tournament in tournaments" v-bind:key="tournament.id" @click="set_tournament(tournament)" class="card" style="cursor: pointer">
                            <div class="card-body">
                                <h5 class="card-title">{{ tournament.id }}</h5>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            
            <div v-if="tournament != null && match == null" class="col">
                <h4>Choose PUBG match:</h4>

                <div class="row pt-3 pb-3">
                    <div v-for="match in matches" v-bind:key="match.id" class="card mb-2" style="cursor: pointer; width: 100%">
                        <PUBGMatch
                                :id = "match.id"
                                :platform = "selected_platform"
                                @click.native="update_match(match.id)"
                            >
                        </PUBGMatch>  
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
    import Config from '../config'
    import PUBGMatch from './pubg/Match.vue'

    export default {
        components: {
            PUBGMatch
        },
        name: 'PUBGChooser',
        data: function () {
            return {
                tournament: null,
                match: null,
                tournaments: [],
                matches: null,
                players: null,
                errors: null,
                mode: "custom_match",
                platforms: [
                    "steam",
                    "psn",
                    "xbox",
                    "kakao"
                ],
                selected_platform: "steam",
                username: ""
            }
        },
        methods: {            
            get_tournaments() {
                this.$api.pubg_get_tournaments().then(tournaments => {
                    this.$data.tournaments = tournaments
                })
            },
            set_tournament(tournament) {
                this.$data.tournament = tournament
                this.$emit('update_pubg_tournament', tournament)
                this.$emit('update_pubg_match', null)

                this.$api.pubg_get_tournament_matches(tournament.id).then((matches) => {
                    this.$data.matches = matches
                })
            },
            search_player() {
                if (this.$data.username == null || this.$data.username == "")
                    return
                
                this.$data.errors = null
                this.$data.players = null
                this.$data.matches = null

                this.$api.pubg_get_players(this.$data.selected_platform, this.$data.username).then(players => {
                    this.$data.players = players
                }).catch(errors => {
                    this.$data.errors = errors
                }) 
            },
            get_matches_by_player(player) {
                this.$data.matches = player.relationships.matches.data
            },
            update_match(match_id) {
                this.$api.pubg_get_match(this.$data.selected_platform, match_id).then((match) => {
                    this.$emit('update_pubg_match', match)
                })
            },
        },
        mounted: function () {
            this.get_tournaments()
        }
    }
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
<template>                
    <div class="row">
        <div>
            <ul class="nav nav-tabs">
                <li class="nav-item">
                    <a class="nav-link active" href="#">Tournament</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link disabled" href="#">Custom match</a>
                </li>
            </ul>
        </div>

        <div>
            <div v-if="tournament == null" class="col">
                <h4>Choose PUBG tournament:</h4>

                <div class="row pt-3 pb-3">
                    <div class="card-columns">
                        <div v-for="tournament in tournaments" v-bind:key="tournament.id" @click="setPUBGTournament(tournament)" class="card" style="cursor: pointer">
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
                    <div v-for="match in matches" v-bind:key="match.id" @click="setPUBGMatch(match)" class="card mb-2" style="cursor: pointer; width: 100%">
                        <div class="card-body">
                            <h5 class="card-title">#{{ match.index}} - Created at {{ match.attributes.createdAt }}</h5>
                            <span>{{ match.id }}</span>
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
        name: 'PUBGChooser',
        props: {
            accesstoken: String
        },
        data: function () {
            return {
                tournament: null,
                match: null,
                tournaments: [],
                matches: []
            }
        },
        methods: {            
            getPUBGTournaments: function () {
                this.$http.get(Config.API_URL + '/pubg/tournaments', {
                    headers: {
                        'Authorization': 'Bearer ' + this.accesstoken
                    }
                }).then(response => {
                    this.$data.tournaments = response.body.data
                })
            },
            getPUBGMatches: function (tournament_id) {
                this.$http.get(Config.API_URL + '/pubg/tournaments/' + tournament_id, {
                    headers: {
                        'Authorization': 'Bearer ' + this.accesstoken
                    }
                }).then(response => {
                    this.$data.matches = response.body.included.sort((a, b) => {
                        return a.attributes.createdAt > b.attributes.createdAt ? 1 : -1
                    })

                    this.$data.matches.forEach((element, index) => {
                        element.index = index + 1
                    });

                    this.$data.matches.reverse()
                })
            },
            setPUBGTournament: function (tournament) {
                this.$data.tournament = tournament
                this.update_pubg()

                this.getPUBGMatches(tournament.id)
            },
            setPUBGMatch: function (match) {
                this.$data.match = match
                this.update_pubg()
            },
            update_pubg: function () {
                this.$emit('update_pubg', {
                    tournament: this.$data.tournament,
                    match: this.$data.match
                })
            }
        },
        mounted: function () {
            this.update_pubg()
            this.getPUBGTournaments()
        }
    }
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
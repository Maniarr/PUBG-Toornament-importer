<template>
    <div>
        <div class="row pt-3">
            <div class="col">
                <h5>Preview:</h5>
                <table class="table table-striped">
                    <thead>
                        <tr>
                        <th scope="col">#</th>
                        <th scope="col">Name</th>
                        <th scope="col">In-game rank</th>
                        <th scope="col">Kills</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr v-for="team in teams" v-bind:key="team.team.number">
                            <th scope="row">{{ team.team.number }}</th>
                            <td>{{ team.team.name }}</td>
                            <td>{{ team.properties.ingame_rank }}</td>
                            <td>{{ team.properties.kills }}</td>
                        </tr>
                    </tbody>
                </table>
                <div class="row text-center justify-content-center">
                    <div class="col">
                        <button @click="confirm_preview" class="btn btn-success">Confirm</button>
                    </div>
                    <div class="col">
                        <button @click="$emit('cancel_preview')" class="btn btn-danger">Cancel</button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
    import Config from '../config'

    export default {
        name: 'ChooserPreview',
        props: {
            accesstoken: String,
            toornament: Object,
            pubg: Object
        },
        data: function () {
            return {
                teams: []
            }
        },
        methods: {
            get_preview: function () {
                this.$http.get(Config.API_URL + '/preview', {
                    headers: {
                        'Authorization': 'Bearer ' + this.accesstoken
                    },
                    params: {
                        toornament_tournament_id: this.$props.toornament.tournament.id,
                        toornament_match_id: this.$props.toornament.match.id,
                        pubg_match_id: this.$props.pubg.match.id
                    }
                }).then(response => {
                    this.$data.teams = response.body.teams

                    this.$data.teams.sort((a, b) => {
                        return a.properties.ingame_rank > b.properties.ingame_rank
                    })
                })
            },
            confirm_preview: function () {
                this.$http.post(Config.API_URL + '/import', {
                        toornament_tournament_id: this.$props.toornament.tournament.id,
                        toornament_match_id: this.$props.toornament.match.id,
                        toornament_game: this.$props.toornament.game,
                        pubg_match_id: this.$props.pubg.match.id,
                    }, {
                        headers: {
                            'Authorization': 'Bearer ' + this.accesstoken
                        }
                    }
                ).then(() => {
                    this.$emit('cancel_preview')
                })
            }
        },
        mounted: function () {
            this.get_preview()
        }
    }
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
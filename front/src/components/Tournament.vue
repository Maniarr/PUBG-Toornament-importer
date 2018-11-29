<template>
    <div>
        <nav class="navbar navbar-light bg-light justify-content-between">
            <a class="navbar-brand" href="#">PUBG importer</a>
            <button class="btn btn-danger" @click="logout">Logout</button>
        </nav>

        <div class="row pt-4">
            <div class="col col-2">
                <div v-if="toornament != null">
                    <h5>Choosen Toornament info:</h5>
                    <div v-if="toornament.tournament != null">
                        <p><b>Tournament:</b></p>
                        <p>{{ toornament.tournament.name }}</p>
                    </div>
                    <div v-if="toornament.game != null">
                        <p><b>Game:</b></p>
                        <p>#{{ toornament.game }}</p>
                    </div>
                </div>
                <div v-if="pubg != null && is_toornament_ready()">
                    <h5>Choosen PUBG info:</h5>
                    <div v-if="pubg.tournament != null">
                        <p><b>Tournament:</b></p>
                        <p>{{ pubg.tournament.id }}</p>
                    </div>
                    <div v-if="pubg.match != null">
                        <p><b>Match:</b></p>
                        <p>#{{ pubg.match.index }} - {{ pubg.match.attributes.createdAt }}</p>
                    </div>
                </div>
            </div>
            <div class="col col-10">
                <ToornamentChooser 
                    v-if="!is_toornament_ready()"
                    ref="toornament"
                    :accesstoken="accesstoken"
                    v-on:update_toornament="update_toornament"
                >
                </ToornamentChooser>
                <PUBGChooser
                    v-if="is_toornament_ready()"
                    :accesstoken="accesstoken"
                    v-on:update_pubg="update_pubg"
                >
                </PUBGChooser>
            </div>
        </div>
    </div>
</template>

<script>
    import Config from '../config'
    import ToornamentChooser from './ToornamentChooser.vue'
    import PUBGChooser from './PUBGChooser.vue'

    export default {
        name: 'Tournament',
        components: {
            ToornamentChooser,
            PUBGChooser
        },
        props: {
            accesstoken: String
        },
        data: function () {
            return {
                toornament: null,
                pubg: null
            }
        },
        methods: {
            logout: function () {
                window.location = '/'
            },
            update_toornament: function (toornament) {
                this.$data.toornament = toornament
            },
            update_pubg: function (pubg) {
                this.$data.pubg = pubg
            },
            is_toornament_ready: function () {
                if (this.$data.toornament == null)
                    return false

                return this.$data.toornament.game != null
            }
        },
        mounted: function () {
        }
    }
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
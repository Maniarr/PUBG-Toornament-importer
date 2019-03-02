<template>
    <div>
        <div class="row pt-4">
            <div class="col col-2">
                <div>
                    <h5>Choosen Toornament info:</h5>
                    <div>
                        <p><b>Tournament:</b></p>
                        <p>{{ toornament.tournament != null ? toornament.tournament.name : "N/A" }}</p>
                    </div>
                    <div>
                        <p><b>Game:</b></p>
                        <p>{{ toornament.game != null ? "#" + toornament.game : "N/A" }}</p>
                    </div>
                </div>
                <div>
                    <h5>Choosen PUBG info:</h5>
                    <div>
                        <p><b>Tournament:</b></p>
                        <p>{{ pubg.tournament != null ? pubg.tournament.id : "N/A" }}</p>
                    </div>
                    <div>
                        <p><b>Match:</b></p>
                        <p>{{ pubg.match != null ? pubg.match.map_name : "N/A" }}</p>
                    </div>
                </div>
            </div>
            <div class="col col-10">
                <ToornamentChooser 
                    v-if="!is_toornament_ready()"
                    ref="toornament"
                    v-on:update_toornament="update_toornament"
                >
                </ToornamentChooser> 
                <PUBGChooser
                    v-if="is_toornament_ready() && !is_preview_ready()"
                    v-on:update_pubg_tournament="update_pubg_tournament"
                    v-on:update_pubg_match="update_pubg_match"
                >
                </PUBGChooser>
                <ChooserPreview
                    v-if="is_preview_ready()"
                    :toornament="toornament"
                    :pubg="pubg"
                    v-on:cancel_preview="cancel"
                >
                </ChooserPreview>
            </div>
        </div>
    </div>
</template>

<script>
    import Config from '../config'
    import ToornamentChooser from './ToornamentChooser.vue'
    import PUBGChooser from './PUBGChooser.vue'
    import ChooserPreview from './ChooserPreview.vue'

    export default {
        name: 'Tournament',
        components: {
            ToornamentChooser,
            PUBGChooser,
            ChooserPreview
        },
        data: function () {
            return {
                toornament: {
                    tournament: null,
                    game: null
                },
                pubg: {
                    tournament: null,
                    match: null
                }
            }
        },
        methods: {
            update_toornament(toornament) {
                this.$data.toornament = toornament
            },
            update_pubg: function (pubg) {
                this.$data.pubg = pubg
            },
            update_pubg_tournament(tournament) {
                this.$data.pubg.tournament = tournament
            },
            update_pubg_match(match) {
                this.$data.pubg.match = match
            },
            is_toornament_ready() {
                if (this.$data.toornament == null)
                    return false

                return this.$data.toornament.game != null
            },
            is_preview_ready() {
                if (this.$data.pubg == null)
                    return false

                return this.is_toornament_ready() && this.$data.pubg.match != null
            },
            cancel() {
                Object.assign(this.$data, this.$options.data.call(this));
            }
        }
    }
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>

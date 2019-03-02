<template>                
    <div class="card mb-2" style="cursor: pointer">
        <div class="card-body">
            <h5 class="card-title">{{ info.map_name }} - {{ info.nb_teams }} teams</h5>
            <span>Mode: {{ info.game_mode }}</span><br>
            <span>Played at {{ info.played_at }}</span>
        </div>
    </div>        
</template>

<script>
    import Config from '../../config'

    export default {
        name: 'PUBGMatch',
        props: {
            id: String,
            platform: String
        },
        data: function () {
            return {
                info: {
                    played_at: null,
                    map_name: null,
                    game_mode: null,
                    nb_teams: null,
                    is_custom: null
                }
            }
        },
        methods: {            
            get_match: function () {
                this.$api.pubg_get_match(this.$props.platform, this.$props.id).then((match) => {
                    this.$data.info = match
                })
            }
        },
        created: function () {
            this.get_match()
        }
    }
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
import Config from '../config.js'

class Api {
    constructor(http, config) {
        this.http = http
        this.token = null
        this.is_connected = false

        this.http.options.root = config.API_URL
    }

    is_connected() {
        return this.is_connected
    }
 
    get_login_url() {
        return this.http.get('login')
    }

    login(code, state) {
        return new Promise((resolve, reject) => {
            this.http.post('login', { code: code, state: state}).then(response => {
                this.token = response.body.access_token
                this.is_connected = true

                resolve()
            }).catch(error => {
                reject(error)
            })
        })
    }

    toornament_get_tournaments() {
        return new Promise((resolve, reject) => {
            this.http.get('tournaments', {
                headers: {
                    'Authorization': 'Bearer ' + this.token
                }
            }).then(response => {
                resolve(response.body)
            }).catch(error => {
                reject(error)
            })
        })
    }

    toornament_get_matches(tournament_id) {
        return new Promise((resolve, reject) => {
            this.http.get('tournaments/' + tournament_id + '/matches', {
                headers: {
                    'Authorization': 'Bearer ' + this.token
                }
            }).then(response => {
                resolve(response.body)
            }).catch(error => {
                reject(error)
            })
        })
    }

    toornament_get_games(tournament_id, match_id) {
        return new Promise((resolve, reject) => {
            this.http.get('tournaments/' + tournament_id + '/matches/' + match_id, {
                headers: {
                    'Authorization': 'Bearer ' + this.token
                }
            }).then(response => {
                resolve(response.body)
            }).catch(error => {
                reject(error)
            })
        })
    }

    pubg_get_tournaments() {
        return new Promise((resolve, reject) => {
            this.http.get('pubg/tournaments', {
                headers: {
                    'Authorization': 'Bearer ' + this.token
                }
            }).then(response => {
                resolve(response.body.data)
            }).catch(error => {
                reject(error)
            })
        })
    }

    pubg_get_tournament_matches(tournament_id) {
        return new Promise((resolve, reject) => {
            this.http.get('pubg/tournaments/' + tournament_id, {
                headers: {
                    'Authorization': 'Bearer ' + this.token
                }
            }).then(response => {
                resolve(response.body.included)
            }).catch(error => {
                reject(error)
            })
        })
    }

    pubg_get_players(platform, username) {
        return new Promise((resolve, reject) => {
            this.http.get('pubg/players', {
                headers: {
                    'Authorization': 'Bearer ' + this.token
                },
                params: {
                    platform: platform,
                    username: username
                }
            }).then(response => {
                if (response.body.errors != null)
                    reject(response.body.errors)
                else
                    resolve(response.body.data)
            }).catch(error => {
                reject(error)
            })
        })        
    }

    pubg_get_match(platform, match_id) {
        return new Promise((resolve, reject) => {
            this.http.get('pubg/matches/' + platform + '/' + match_id, {
                headers: {
                    'Authorization': 'Bearer ' + this.token
                }
            }).then(response => {
                let match_info = response.data.data
            
                resolve({
                    id: match_info.id,
                    platform: platform,
                    played_at: match_info.attributes.createdAt,
                    map_name: match_info.attributes.mapName,
                    game_mode: match_info.attributes.gameMode,
                    nb_teams: match_info.relationships.rosters.data.length
                })
            }).catch(error => {
                reject(error)
            })
        })
    }

    get_preview(toornament_tournament_id, toornament_match_id, pubg_platform, pubg_match_id) {
        return new Promise((resolve, reject) => {
            this.http.get('preview', {
                headers: {
                    'Authorization': 'Bearer ' + this.token
                },
                params: {
                    toornament_tournament_id: toornament_tournament_id,
                    toornament_match_id: toornament_match_id,
                    pubg_platform: pubg_platform,
                    pubg_match_id: pubg_match_id
                }
            }).then(response => {
                resolve(response.body.teams)
            }).catch(error => {
                reject(error)
            })
        })
    }

    import_preview(toornament_tournament_id, toornament_match_id, toornament_game, pubg_platform, pubg_match_id) {
        return new Promise((resolve, reject) => {
            this.http.post('import', {
                toornament_tournament_id: toornament_tournament_id,
                toornament_match_id: toornament_match_id,
                toornament_game: toornament_game,
                pubg_platform: pubg_platform,
                pubg_match_id: pubg_match_id
            }, {
                headers: {
                        'Authorization': 'Bearer ' + this.token
                }
            }).then(response => {
               resolve(response)
            }).catch(error => {
                reject(error)
            })
        })
    }
}

export default {
    install(Vue, options) {
        Vue.prototype.$api = new Api(Vue.http, Config)
    }
}

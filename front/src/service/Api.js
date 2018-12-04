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
}

export default {
    install(Vue, options) {
        Vue.prototype.$api = new Api(Vue.http, Config)
    }
}

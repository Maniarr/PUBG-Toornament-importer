<template>
  <div id="app">
    <Login v-if="link != null" :link="link" />
    <Tournament v-if="accesstoken != null" :accesstoken="accesstoken" />
  </div>
</template>

<script>
  import Login from './components/Login.vue'
  import Tournament from './components/Tournament.vue'
  import Config from './config'

  export default {
    name: 'app',
    components: {
      Login,
      Tournament
    },
    data: function () {
      return {
        link: null,
        accesstoken: null
      }
    },
    mounted: function () {
      let url = new URL(window.location);

      let code = url.searchParams.get("code");
      let state = url.searchParams.get("state");

      if (code === null || state === null) {
        this.$http.get(Config.API_URL + '/login').then(response => {
          this.$data.link = response.body.connection_uri
        })

        return;
      }

      this.$http.post(Config.API_URL + '/login', { code: code, state: state }).then(response => {
        this.$data.accesstoken = response.body.access_token
      }, error => {
        window.location = "/"
      })
    }
  }
</script>

<style>
</style>
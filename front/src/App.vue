<template>
  <div id="app">
    <div v-if="!is_connected">
      <nav class="navbar navbar-expand-lg navbar-light bg-light">
          <a class="navbar-brand" href="#">PUBG importer</a>
          <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
              <span class="navbar-toggler-icon"></span>
          </button>
      </nav>

      <table style="height: 80vh; width: 100%" class="text-center">
          <tbody>
              <tr>
                  <td class="align-middle" v-if="link != null">
                      <a class="btn btn-primary" v-bind:href="link">Login with Toornament</a>
                  </td>
              </tr>
          </tbody>
      </table>
    </div>
    <Tournament v-else/>
  </div>
</template>

<script>
  import Tournament from './components/Tournament.vue'
  import Config from './config'

  export default {
    name: 'app',
    components: {
      Tournament
    },
    data() {
      return {
        link: null,
        is_connected: false
      }
    },
    mounted() {
      let url = new URL(window.location);

      let code = url.searchParams.get("code");
      let state = url.searchParams.get("state");

      if (code === null || state === null) {
        this.$api.get_login_url().then(response => {
          this.$data.link = response.body.connection_uri
        })

        return;
      }

      this.$api.login(code, state).then(() => {
        this.is_connected = true
      }).catch(error => {
        window.location = "/"
      })
    }
  }
</script>

<style>
</style>
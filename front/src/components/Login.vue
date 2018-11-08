<template>
    <div>
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
</template>

<script>
import Config from '../config'

export default {
  name: 'Login',
  props: {
    link: String
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>

</style>

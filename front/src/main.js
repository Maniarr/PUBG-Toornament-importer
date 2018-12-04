import Vue from 'vue'
import VueResource from 'vue-resource'
import App from './App.vue'
import Api from './service/Api.js'

Vue.use(VueResource)

Vue.config.productionTip = false

Vue.http.interceptors.push((request, next) => {
  request.credentials = true;
  next();
});

Vue.use(Api)

new Vue({
  render: h => h(App),
}).$mount('#app')

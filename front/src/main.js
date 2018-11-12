import Vue from 'vue'
import VueResource from 'vue-resource'
import App from './App.vue'

Vue.use(VueResource)

Vue.config.productionTip = false

Vue.http.interceptors.push((request, next) => {
  request.credentials = true;
  next();
});

new Vue({
  render: h => h(App),
}).$mount('#app')

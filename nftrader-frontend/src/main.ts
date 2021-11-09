import Vue from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify'
import SocketService from './services/SocketService';

Vue.config.productionTip = false
Vue.prototype.$ws = new WebSocket('ws://127.0.0.1:3030/feed');
Vue.prototype.$wsService = new SocketService();

Vue.prototype.$ws.onopen = function() {
  console.log("WebSocket is open now.");
};

Vue.prototype.$ws.onmessage = function(data: any) {
    Vue.prototype.$wsService.handle_message(data);
}

new Vue({
  vuetify,
  render: h => h(App)
}).$mount('#app')

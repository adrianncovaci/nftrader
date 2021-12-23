import Vue from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify'
import router from './router'
import SocketService from './services/SocketService';
//import { Message } from './types/proto';

Vue.config.productionTip = false
Vue.prototype.$ws = new WebSocket('ws://127.0.0.1:4040');
Vue.prototype.$wsService = new SocketService();
Vue.prototype.$feed = [];
Vue.prototype.$ws.onopen = function() {
    console.log("WebSocket is open now.");
};

Vue.prototype.$ws.onmessage = function(data: any) {
    Vue.prototype.$wsService.handle_message(data);
}

Vue.config.productionTip = false

new Vue({
    vuetify,
    router,
    render: h => h(App)
}).$mount('#app')

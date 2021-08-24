import { createApp } from 'vue';
import vuetify from './plugins/vuetify';
import App from './App.vue';
import router from './router';
import store from './store';

createApp(App)
  .use(router)
  .use(store)
  .use(vuetify)
  .mount('#app');

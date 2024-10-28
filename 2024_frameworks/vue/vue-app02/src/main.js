import { createApp } from 'vue'
import App from './App.vue'

import directives from "./directives/directives.js";

const app = createApp(App);

/*
const focusDirective = {
  mounted(el) {
    el.focus();
  }
}

app.directive("focus", focusDirective);
*/

for (let name in directives) {
  // Creation of the directive name within the application
  app.directive(name, directives[name]);
}

app.mount('#app')

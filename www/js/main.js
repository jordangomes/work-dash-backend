// js/main.js

const { createApp } = Vue;
const { loadModule } = window['vue3-sfc-loader'];

const options = {
  moduleCache: {
    vue: Vue,
  },
  getFile(url) {
    return fetch(url).then((resp) =>
      resp.ok ? resp.text() : Promise.reject(resp)
    );
  },
  addStyle(styleStr) {
    const style = document.createElement('style');
    style.textContent = styleStr;
    const ref = document.head.getElementsByTagName('style')[0] || null;
    document.head.insertBefore(style, ref);
  },
  log(type, ...args) {
    console.log(type, ...args);
  },
};

const app = createApp({
  components: {

    DashHeader: Vue.defineAsyncComponent(() =>
      loadModule('js/components/dash-header.vue', options)
    ),
    Dashboard: Vue.defineAsyncComponent(() =>
        loadModule('js/pages/dashboard.vue', options)
    ),
  },
}).mount('#app');
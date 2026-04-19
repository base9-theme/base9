import { createApp } from 'vue';
import { create } from 'naive-ui';
import router from './router';
import App from './App.vue';
import init from 'base9-builder';
// import Antd from 'ant-design-vue'
// import { ConfigProvider } from 'ant-design-vue'
// import { createVuetify } from 'vuetify'
// import 'ant-design-vue/dist/antd.variable.min.css'
// import 'vuetify/styles' // Global CSS has to be imported
// import * as components from 'vuetify/components'
// import * as directives from 'vuetify/directives'
import Markdown from 'vue3-markdown-it';

function x() {}

async function main() {
  try {
    await init()
  } catch(e) {
    console.log('dev mode');
    const input = new URL('/node_modules/base9-builder/base9_builder_bg.wasm', import.meta.url);
    await init(input)
  }

  // hack, uncomment during dev

  // uncomment during prod

  const naive = create();
  // const vuetify = createVuetify({
  //   components,
  //   directives,
  //   theme: {
  //     defaultTheme: 'base9',
  //     themes: {
  //       base9: {
  //         dark: false,
  //         colors: {
  //           background: '#FFFFFF',
            
  //           surface: '#FFFFFF',
  //           primary: '#6200EE',
  //           'primary-darken-1': '#3700B3',
  //           secondary: '#03DAC6',
  //           'secondary-darken-1': '#018786',
  //           error: '#B00020',
  //           info: '#2196F3',
  //           success: '#4CAF50',
  //           warning: '#FB8C00',
  //         }
  //       }
  //     }
  //   }
  // });
  createApp(App)
    .use(naive)
    // .use(Antd)
    // .use(vuetify)
    .use(router)
    .use(Markdown)
    .mount('#app');
}

main();

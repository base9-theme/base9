import { defineConfig } from 'vite';
import markdownItAnchor from "markdown-it-anchor";
import vue from '@vitejs/plugin-vue';
import content from '@originjs/vite-plugin-content';
import rawPlugin from 'vite-raw-plugin';
import AutoImport from 'unplugin-auto-import/vite';
import Components from 'unplugin-vue-components/vite';
import { NaiveUiResolver } from 'unplugin-vue-components/resolvers';
import Markdown from 'vite-plugin-md'

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    rollupOptions: {
      input: {
        main: '404.html',
        // nested: resolve(__dirname, 'nested/index.html')
      },
    },
  },
  server: {
    host: true,
    port: 8080,
    fs: {
      allow: [".."], // allow loading local wasm in other folders
    }
  },
  plugins: [
    vue({
      include: [/\.vue$/, /\.md$/],
    }),
    Markdown({
      markdownItSetup(md) {
        md.use(markdownItAnchor)
      },
    }),
    content(),
    rawPlugin({ fileRegex: /\.(mustache)$/ }),
    AutoImport({
      resolvers: [NaiveUiResolver()],
    }),
    Components({
      resolvers: [NaiveUiResolver()],
    }),
  ],
});

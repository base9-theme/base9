import { createRouter, createWebHistory } from 'vue-router';
import Preview from './views/Preview.vue';
import Preview2 from './views/Preview2.vue';
import Development from './views/Development.vue';
import Export from './views/Export.vue';
import Landing from './views/Landing.vue';
import Pick from './views/Pick.vue';
import Build from './views/Build.vue';
import about from './assets/about.md';
import colorPaletteGuide from './assets/color_palette_guide.md';

const routes = [
  {
    path: '/',
    name: 'Landing',
    component: Landing,
  },
  {
    path: '/pick',
    name: 'Pick Curated Themes',
    component: Pick,
  },
  {
    path: '/build',
    name: 'Build new Theme',
    component: Build,
  },
  {
    path: '/preview',
    name: 'Preview',
    component: Preview2,
  },
  {
    path: '/install',
    name: 'Install',
    component: Export,
  },
  {
    path: '/dev',
    name: 'Development',
    component: Development,
  },
  {
    path: '/guide/color_palette',
    name: 'Color Palette',
    component: colorPaletteGuide,
  }
];

export default createRouter({
  history: createWebHistory(),
  routes,
});

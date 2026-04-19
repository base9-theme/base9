<template>
  <n-layout position="absolute" class="main">
    <n-layout-header bordered class="header" >
      <span class="home" @click="$router.push('/')">
        <Logo :size="32" :palette="palette.palette.value"/>
        <!-- <span style="width: 32px; height: 32px;" v-html="svgString"></span> -->
        <span style="font-size: 32px">base9</span>
      </span>
      <n-menu v-model:value="activeKey" mode="horizontal" :options="menuOptions" />
      <!-- <n-anchor-link href="https://github.com"></n-anchor-link> -->
      <!-- <a href="/">
          <n-icon><dice /></n-icon>
      </a> -->
      <!-- <n-button type="primary">
          <n-icon><icon-color-palette /></n-icon>
      </n-button> -->
      <!-- <n-button circle quaternary>
          <n-icon><logo-discord /></n-icon>
      </n-button> -->
      <a href="https://github.com/base9-theme/base9">
        <n-button circle quaternary>
            <n-icon><logo-github /></n-icon>
        </n-button>
      </a>
    </n-layout-header>
    <n-layout-content position="absolute" class="slot">
      <slot/>
    </n-layout-content>
  </n-layout>
</template>
<style scoped>
.main {
  height: 100%;
  /* display: flex;
  flex-flow: column; */
}
.header {
  display: flex;
  height: 64px;
  align-items: center;
}

.home {
  display: flex;
  align-items: center;
  cursor: pointer;
}
.slot {
  top: 64px;
  bottom: 0px;
  /* width: 70vw; */
  background-color: var(--base9-0);
  /* margin-left: 15vw; */
  padding: 1em;
}
.slot {
  /* width: 70vw; */
  background-color: var(--base9-0);
  /* margin-left: 15vw; */
  padding: 1em;
}
</style>
<script setup lang="ts">
import { h, inject, ref } from "vue";
import type { Ref } from "vue";
import type { ColorPalette } from "../helpers";
import { RouterLink } from "vue-router";
import { LogoGithub, LogoDiscord, ColorPalette as IconColorPalette } from '@vicons/ionicons5'
import { Palette } from "../helper2";

const palette = inject('palette') as Ref<Palette>;

const activeKey = ref(null);

function createRouterLinkMenuItem({path, label}: {path: string, label: string}) {
   return {
     label: () => h(
      RouterLink,
      {
        to: { path },
      },
      { default: () => label },
     ),
     key: label,
   }
}
const menuOptions = [
  createRouterLinkMenuItem({path: '/pick', label: 'Pick'}),
  createRouterLinkMenuItem({path: '/build', label: 'Build'}),
  createRouterLinkMenuItem({path: '/preview', label: 'Preview'}),
  createRouterLinkMenuItem({path: '/install', label: 'Install'}),
  // {
  //   label: 'Guide',
  //   key: 'Guide',
  //   children: [
  //     createRouterLinkMenuItem({path: '/guide/color_palette', label: 'Color Palette'}),
  //   ]
  // }
];

const showEditor = ref(false);
const activate = (place: string) => {
      showEditor.value = true
    }
</script>

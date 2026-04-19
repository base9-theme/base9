<template>
<n-config-provider class="main" v-bind:style="cssVariable" :theme-overrides="themeOverrides">
  <n-message-provider>
    <router-view  :key="$route.path" />
  </n-message-provider>
</n-config-provider>
</template>
<style>
a {
  text-decoration: none;
  color: var(--base9-primary-p100);
}
</style>
<style scoped>
.main {
  height: 100vh;
}
</style>
<script setup lang="ts">
import { NConfigProvider, GlobalThemeOverrides } from 'naive-ui'

// This starter template is using Vue 3 <script setup> SFCs
// Check out https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup
import {
  computed, h, provide, reactive, Ref, ref, watch,
} from 'vue';
import hljs from 'highlight.js/lib/core';
import Color from 'color';
import _, { isArray } from 'lodash';
import {
  useRoute, useRouter, RouterLink,
} from 'vue-router';
import type { RouteLocation } from 'vue-router';

import Base16SchemePicker from './components/Base16SchemePicker.vue';
import Import from './components/Import.vue';
import Logo from './components/Logo.vue';
import { toPaletteString, PALETTE_REGEX, getCssVariableName, getCssVariableContrastName, contrastColor, N9, I9, DEFAULT_PALETTE } from './helpers';
import type { ColorPalette, ShadesFormatted } from './helpers';
import logoTemplate from './assets/templates/logo.svg.mustache';

import { renderString, getData } from "base9-builder";
import { Formatted } from "base9-builder";
import { ConfigProvider } from 'ant-design-vue/lib/components';
import { Palette, palette_key } from './helper2';

const route = useRoute();
const router = useRouter();

function getDefaultPalette() {
  const palette = route.params.palette;
  if (!(typeof palette === 'string' && PALETTE_REGEX.test(palette))) {
    return DEFAULT_PALETTE;
  }
  return palette;
}

function hasBase9Param(r: RouteLocation) {
  return r.query.palette !== undefined;
}

router.beforeEach((to, from, next) => {
  if (!hasBase9Param(to) && hasBase9Param(from)) {
    // TODO remove "!"
    next({ name: to.name!, query: from.query });
  } else {
    next();
  }
});
// const palette = ref(getDefaultPalette());
const palette = new Palette(getDefaultPalette());
provide('palette', palette);
provide(palette_key, palette);

watch(
  () => route.query.palette,
  (newPalette) => {
    if (typeof newPalette !== 'string') {
      palette.palette.value = DEFAULT_PALETTE;
      return;
    }

    if(!PALETTE_REGEX.test(newPalette)) {
      return;
    }

    palette.palette.value = newPalette;
  },
);

watch(() => palette.palette.value, async () => {
  await router.push({
    query: { palette: palette.palette.value },
  });
});

const colorData = computed(() => getData(palette.palette.value))
provide('colorData', colorData);
const themeOverrides: Ref<GlobalThemeOverrides> = computed(() => {
  const M = colorData.value;
  const H = (c: Formatted) => `#${c.hex}`;
  // console.log(H(M.primary.p100));
  return {
    common: {
      borderRadius: '0px',
      baseColor: H(M.background),
      bodyColor: H(M.background),
      cardColor: H(M.background),
      primaryColor: H(M.primary.p100),
      primaryColorHover: H(M.primary.p100),
      primaryColorPressed: H(M.primary.p75),
      primaryColorSuppl: H(M.red.p100),
      // primaryColorSuppl: H(M.primary.p50),
      textColor: H(M.foreground.p100),
      textColor1: H(M.foreground.p100),
      textColor2: H(M.foreground.p100),
      dividerColor: H(M.foreground.p25),
      borderColor: H(M.foreground.p50),
      popoverColor: H(M.foreground.p10),
      hoverColor: H(M.primary.p25),
      // borderColor: H(M.background),
      closeIconColor: H(M.foreground.p75),
      closeIconColorHover: H(M.foreground.p100),
      closeIconColorPressed: H(M.foreground.p75),

      clearIconColor: H(M.foreground.p75),
      clearIconColorHover: H(M.foreground.p100),
      clearIconColorPressed: H(M.foreground.p75),
      inputColor: H(M.background),
      // hoverColor: '#FF0000', 
      // borderColor: H(M.red.p100),
      // hoverColor: "#ff0000",
      // actionColor: H(M.red.p100),
    },
    Button: {
      // textColor: H(M.background),
      colorQuaternary: H(M.background),
      colorQuaternaryHover: H(M.foreground.p25),
      colorQuaternaryPressed: H(M.foreground.p10),
    },
    Tag: {
      colorBordered: H(M.background),
      color: H(M.background),
      closeIconColor: H(M.foreground.p75),
      closeIconColorHover: H(M.foreground.p100),
      closeIconColorPressed: H(M.foreground.p75),
      textColorPrimary: H(M.background),
      colorPrimary: H(M.secondary.p75),
    },
    Tooltip: {
      color: H(M.foreground.p10),
      textColor: H(M.foreground.p100),
    }
  };
});

const activeKey = ref(null);

function cssHelper(c: Formatted, i: number): [string, string][] {
  return [
    [`--base9-${i}`, "#"+c.hex],
    [`--base9-${i}-contrast`, contrastColor(Color("#" + c.hex)).hex()],
    // [`--base9-${i}-p10`, "#"+c.p10.hex],
    // [`--base9-${i}-p25`, "#"+c.p25.hex],
    // [`--base9-${i}-p50`, "#"+c.p50.hex],
    // [`--base9-${i}-p75`, "#"+c.p75.hex],
    // [`--base9-${i}-p100`, "#"+c.p100.hex],
    // [`--base9-${i}-p125`, "#"+c.p100.hex],
  ]
}

const cssVariable = computed(() => {
  const M = colorData.value;
  const entries: [string, string][] = _.flatMap(
    _.range(N9),
    i => cssHelper(M.base9[`c${i as I9}`], i));
  M.PROGRAMMABLE.forEach(row => {
    if(row.color) {
      entries.push(["--base9-" + row.path.dotted.replaceAll(".", "-"), "#" + row.color.hex])
    }
  })
  return Object.fromEntries(entries);
});

</script>

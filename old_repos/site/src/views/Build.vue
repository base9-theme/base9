<template>
<Main>
<div class="container">
  <div class="title-bar">
    <n-button size="large" @click="stepI -= 1" class="back" :style="visibleStyle(step.key !== 'bg')" quaternary type="primary">
        <template #icon>
        <n-icon><chevron-back /></n-icon>
        </template>
    </n-button>
    <div class="title-center">
      <div class="title">{{step.title}}</div>
      <!-- <div class="sub-title">{{step.subtitle}}</div> -->
    </div>
    <n-button size="large" @click="stepI += 1" class="forward" :style="visibleStyle(step.key !== 'primary')" quaternary type="primary">
        <template #icon>
        <n-icon><chevron-forward /></n-icon>
        </template>
    </n-button>
  </div>
  <div class="preview">
    <div>
      {{step.subtitle}}
    </div>
    <n-button v-if="step.key === 'primary'" type="primary" @click="clickPreview"
size="large">Generate & Preview</n-button>

  </div>

  <BuildColorPicker v-model:color="color1"></BuildColorPicker>
  <n-button circle size="large" @click="random()" class="forward" quaternary type="primary">
      <template #icon>
      <n-icon><dice /></n-icon>
      </template>
  </n-button>
</div>
</Main>
</template>
<style scoped>
.container {
  display: flex;
  flex-flow: column;
  height: 100%;
  align-items: center;
}
.title-bar {
  display: flex;
  width: 100%;
  align-items: center;
  gap: 2em;
  justify-content: center;;
}
.title-center {
  width: 50%;
  display: flex;
  align-items: center;
  flex-flow: column;
  height: 8em;
}
.title {
  font-size: 2em;
}

.preview {
  flex-grow: 1;
  display: flex;
  flex-flow: column;
  align-items: center;
}

.preview-link {
  text-decoration: none;
}

.hide {
}
</style>
<script setup lang="ts">
import {
  computed, defineComponent, h, inject, nextTick, provide, Ref, ref, watch,
} from 'vue';
import Color from 'color';
import _ from 'lodash';
import { ChevronForward, ChevronBack, Dice } from '@vicons/ionicons5'

import { contrastColor, toPaletteString, getColor } from '../helpers';
import * as base9 from 'base9-builder';
import { NTag } from 'naive-ui/lib/tag';
import { themes, TAGS } from '../themes/index';
import { Tag, Theme } from '../themes/index';
import { useRouter } from 'vue-router';
// import type { AutoCompleteInst } from 'naive-ui'
import type { ColorPalette } from '../helpers';
import type {
  ComputedRef
} from 'vue';
import BuildColorPicker from '../components/BuildColorPicker.vue';
import { renderString } from 'base9-builder';
import { Palette } from '../helper2';

const router = useRouter();
const palette = inject<Palette>('palette')!;

const STEPS = [
  {
    key: 'bg',
    title: '1. Background',
    subtitle: 'The background color throughout the theme.\nShould be close to white for light themes and close to black for dark theme.',
  },
  {
    key: 'fg',
    title: '2. Foreground',
    subtitle: 'The foreground color throughout the theme. Used in normal text.\nVarious shades will also used for design elements, such as divider.',
  },
  {
    key: 'primary',
    title: '3. Hues',
    subtitle: 'Primary: the most common color that apears throughout the theme. Used in buttons, highlighting selected texts, etc.\n' +
    'After done, preview the theme and make adjustments for less used colors.' +
    '',
  },
  {
    key: 'tweak',
    title: 'Step 4',
    subtitle: 'Tweak all colors.',
  },
] as const;
type Step = typeof STEPS[number];
const stepI = ref(0);
const step = computed(() => STEPS[stepI.value]);

function visibleStyle(b: boolean) {
  return {
    visibility: b ? 'visible' : 'hidden',
  }
}

function random() {
  const list = palette.palette.value.split('-');
  switch(step.value.key) {
    case 'bg':
      color1.value = Color(renderString('?', '#{{background.hex}}'));
      break;
    case 'fg':
      color1.value = Color(renderString(`${list[0]}-?`, '#{{foreground.p100.hex}}'));
      break;
    case 'primary':
      const newList = renderString(`${list[0]}-${list[1]}-?`, '{{PALETTE}}').split('-');
      newList[3] = newList[2];
      newList[4] = newList[2];
      newList[5] = newList[2];
      newList[6] = newList[2];
      newList[7] = newList[2];
      newList[8] = newList[2];
      palette.palette.value = newList.join('-');
  }
}

const color1 = computed({
  // getter
  get(): Color {
    switch(step.value.key) {
      case 'bg':
        return getColor(palette.palette.value, 0);
      case 'fg':
        return getColor(palette.palette.value, 1);
      case 'primary':
        return getColor(palette.palette.value, 2);
      case 'tweak':
        return getColor(palette.palette.value, 2);
    }
  },
  // setter
  set(c: Color) {
    const hex1 = c.hex().substring(1);
    const hex2 = contrastColor(c).hex().substring(1);
    const bg = palette.hexs[0].value;
    const fg = palette.hexs[1].value;
    const c1 = palette.hexs[2].value;
    const c2 = palette.hexs[3].value;
    // Note: we are using destructuring assignment syntax here.
    switch(step.value.key) {
      case 'bg':
        palette.palette.value = `${hex1}-${hex2}-${hex2}-${hex2}-${hex2}-${hex2}-${hex2}-${hex2}-${hex2}`;
        break;
      case 'fg':
        palette.palette.value = `${bg  }-${hex1}-${hex1}-${hex1}-${hex1}-${hex1}-${hex1}-${hex1}-${hex1}`;
        break;
      case 'primary':
        palette.palette.value = `${bg  }-${fg  }-${hex1}-${hex1}-${hex1}-${hex1}-${hex1}-${hex1}-${hex1}`;
        break;
      case 'tweak':
        break;
    }
  }
});

function clickPreview() {
  const newPalette = renderString(`${palette.palette.value.substring(0,21)}?`,
"{{PALETTE}}");
  router.push({
    path: '/preview',
    query: { palette: newPalette },
  });

}
</script>

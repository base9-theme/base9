<template>
  <div>
    <h2>Basic Contrast</h2>
    <div
      class="preview-list"
      :style="{
        'grid-template-columns': '1fr '.repeat(previewListColumns.length),
      }"
    >
      <div
        class="preview-cell"
        :style="{
          background: colorData.background,
          color: colorData.c0.p100,
        }"
        v-for="([s], i) in previewListColumns"
        v-bind:key="i"
      >
        {{ s }}
      </div>
      <div
        class="preview-cell"
        :style="{ background: bg.hex(), color: fg.hex() }"
        v-for="({ bg, fg }, i) in previewList"
        v-bind:key="i"
      >
        {{ previewNumber(bg, fg).toFixed(1) }}
      </div>
    </div>
    <h2>ANSI Colors</h2>
    <div class="ansi-list">
      <div
        class="ansi-cell"
        v-for="i in _.range(8)"
        v-bind:key="i"
      >
        {{ ansiLabel[i] }}
      </div>
      <div
        class="ansi-cell"
        :style="{ background: c }"
        v-for="(c, i) in ansiList"
        v-bind:key="i"
      >
      </div>
    </div>
  </div>
</template>
<style scoped>
.preview-list {
  display: grid;
  column-gap: 2px;
  background: var(--base9-0);
}
.preview-cell {
  height: 20px;
  text-align: center;
}
.ansi-list {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr;
}
.ansi-cell {
  height: 20px;
  text-align: center;
}
</style>
<script setup lang="ts">
import {
  computed, defineComponent, inject, Ref, ref,
} from 'vue';
import Color from 'color';
import _ from 'lodash';
// import Ansi from './Ansi.vue';

// import tmp from '..assets'
import { toPaletteString } from '../helpers';
import type { ColorPalette } from '../helpers';
import * as base9 from 'base9-builder';

const previewListRows = ['0', '1', '2', '3', '4', '5', '6', '7'] as const;
type PreviewListRowsType = (typeof previewListRows)[number];
type x = `c${PreviewListRowsType}`;
const previewListColumns: [
  string,
  (c: PreviewListRowsType, colors: base9.Colors<string>) => string,
  (c: PreviewListRowsType, colors: base9.Colors<string>) => string,
][] = [
  ['p10 bg', (c, colors) => colors[`c${c}`].p10, (c, colors) => colors.foreground.p100],
  ['p25 bg', (c, colors) => colors[`c${c}`].p25, (c, colors) => colors.foreground.p100],
  ['p50 bg', (c, colors) => colors[`c${c}`].p50, (c, colors) => colors.foreground.p100],
  ['p75 inv.', (c, colors) => colors[`c${c}`].p75, (c, colors) => colors.background],
  ['p75', (c, colors) => colors.background, (c, colors) => colors[`c${c}`].p75],
  ['p100', (c, colors) => colors.background, (c, colors) => colors[`c${c}`].p100],
  ['p125', (c, colors) => colors.background, (c, colors) => colors[`c${c}`].p125],
];

// function getVar(i: number) {
//   return `var(${getCssVariableName(i)})`;
// }

// const cellStyles = _.times(N, (i) => ({
//   'background-color': getVar(i),
//   color: getVar(contrastIndex[i]),
// }));
const colors = inject('colors') as Ref<ColorPalette>;
const colorData = computed(() => base9.getColors(toPaletteString(colors.value)));
// const namedColors = computed(() => {
//   const tmp = getColorData(colors.value);
//   (window as any).namedColors = tmp;
//   return tmp;
// });
const previewList = computed(() => _.flatMap(previewListRows, (c) => _.map(previewListColumns, ([s, bg, fg]) => {
  return {
    bg: Color(bg(c, colorData.value)),
    fg: Color(fg(c, colorData.value)),
  };
})));
function previewNumber(bg: Color, fg: Color) {
  return fg.contrast(bg);
}

const ansiList = computed(() => _.times(
  16,
  (x) => (colorData.value.ansi as any)["c"+x.toString(16)],
));
const ansiLabel = [
  'Black',
  'Red',
  'Green',
  'Yellow',
  'Blue',
  'Magenta',
  'Cyan',
  'White',
  'Bright Black',
  'Bright Red',
  'Bright Green',
  'Bright Yellow',
  'Bright Blue',
  'Bright Magenta',
  'Bright Cyan',
  'Bright White',
];
</script>


<template>
<div
    class="list"
    :style="{ 'grid-template-columns': `repeat(${previewListColumns.length}, 1fr)`, }"
>
    <div
        class="cell"
        v-for="([s], i) in previewListColumns"
        v-bind:key="i"
    >
        {{ s }}
    </div>
    <div
        class="cell"
        :style="{ background: bg, color: fg }"
        v-for="({ bg, fg, contrast }, j) in previewList"
        v-bind:key="j"
    >
        {{ contrast }}
    </div>
</div>
</template>
<style scoped>
.list {
  display: grid;
  column-gap: 2px;
  background: var(--base9-0);
  grid-template-columns: repeat(9, 1fr);
}
.cell {
  height: 20px;
  text-align: center;
}

</style>
<script setup lang="ts">
import { Formatted } from "base9-builder";
import Color from "color";
import _ from "lodash";
import { computed, inject } from "vue";
import { palette_key } from "../../helper2";
const palette = inject(palette_key)!;
const previewListRows = ['0', '1', '2', '3', '4', '5', '6', '7'] as const;
type PreviewListRowsType = (typeof previewListRows)[number];
const previewListColumns: [
  string,
  (c: PreviewListRowsType) => Formatted,
  (c: PreviewListRowsType) => Formatted,
][] = [
  ['p10 bg', c => palette.colorData.value[`c${c}`].p10, c => palette.colorData.value.foreground.p100],
  ['p25 bg', c => palette.colorData.value[`c${c}`].p25, c => palette.colorData.value.foreground.p100],
  ['p50 bg', c => palette.colorData.value[`c${c}`].p50, c => palette.colorData.value.foreground.p100],
  ['p75 inv.', c => palette.colorData.value[`c${c}`].p75, c => palette.colorData.value.background],
  ['p100 inv.', c => palette.colorData.value[`c${c}`].p100, c => palette.colorData.value.background],
  ['p75', c => palette.colorData.value.background, c => palette.colorData.value[`c${c}`].p75],
  ['p100', c => palette.colorData.value.background, c => palette.colorData.value[`c${c}`].p100],
  ['p125', c => palette.colorData.value.background, c => palette.colorData.value[`c${c}`].p125],
];
const previewList = computed(() => _.flatMap(previewListRows, (c) => _.map(previewListColumns, ([s, bgf, fgf]) => {
  const bg = `#${bgf(c).hex}`;
  const fg = `#${fgf(c).hex}`;
  return {
    bg,
    fg,
    contrast: Color(bg).contrast(Color(fg)).toFixed(1),
  };
})));
</script>

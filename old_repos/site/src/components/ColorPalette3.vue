<template>
<div class="grid" :style="getStyle(0)">
    <div class="background"></div>
    <div class="foreground" :style="getStyle(1)"></div>
    <div
        class="hue"
        v-for="i in _.range(2, 9)"
        v-bind:key="i"
        :style="getStyle(i)"
    >
    </div>
</div>
</template>
<style scoped>
.container {
  height: 100%;
}
.grid {
  height: 100%;
  position: relative;
  box-sizing: border-box;
  background-color: var(--base9-0);
  color: var(--base9-0-contrast);
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr 1fr;
  grid-template-rows: 1fr 1fr 1fr;
  gap: 0.2em;
  align-items: begin;
  /* justify-items: end; */
}

.grid > div {
  /* box-sizing: border-box;
  margin: 0.5em; */
  display: flex;
  align-items: center;
  justify-content: center;
  /* text-align: center;
  vertical-align: middle; */
}


.background {
    /* position: absolute;
    top: 0; */
    color: var(--base9-0-contrast);
    grid-column: 7 / span 1;
    grid-row: 3 / span 1;
    height: 100%;
    width: 100%;
    /* display: flex;
    align-items: end;
    flex-direction: row-reverse; */
}
.foreground {
    grid-column: 1 / span 7;
    background-color: var(--base9-1);
    color: var(--base9-1-contrast);
    height: 100%;
    /* border-radius: 10px; */
}

.hue {
    /* border-radius: 0.5em; */
    grid-row: 2 / span 2;
}
</style>
<script setup lang="ts">
import * as _ from 'lodash';
import { computed } from 'vue';

const props = defineProps<{
    palette: string
}>();
const LABELS = [
    'background', 'foreground', 'primary', 'secondary', 'tertiary', 'quaternary', 'quinary', 'senary', 'septenary'
] as const;

function getStyle(i: number) {
    if(i < 2) {
        return {
            background: `#${props.palette.substring(i*7, i*7+6)}`,
        };
    } else {
        return {
            background: `#${props.palette.substring(i*7, i*7+6)}`,
            height: `${(12-i)*10}%`,
            gridColumn: `${i-1} / span 1`,
        };
    }
}
</script>

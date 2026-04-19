<template>
<div class="container">
    <div class="grid">
        <div class="background"><PaletteEditorCell :i="0" :locked="locked"></PaletteEditorCell></div>
        <div class="foreground"><PaletteEditorCell :i="1" :locked="locked"></PaletteEditorCell></div>
        <div
            class="hue"
            v-for="i in _.range(2, 9)"
            v-bind:key="i"
            :style="getStyle(i)"
        >
            <PaletteEditorCell :i="i" :locked="locked"></PaletteEditorCell>
        </div>
    </div>
    <div class="bar">
        <n-button @click="generate()" type="primary">
            <template #icon><n-icon><dice /></n-icon></template>
            Generate
        </n-button>
        <n-button @click="toggleDark()" type="primary">
            <n-icon>
                <sunny-outline v-if="!palette.dark.value" />
                <moon-outline v-if="palette.dark.value" />
            </n-icon>
        </n-button>
    </div>
</div>
</template>
<style scoped>
.container {
    border-bottom: 1px solid var(--base9-foreground-p25);
}
.grid {
  height: 16vh;
  position: relative;
  box-sizing: border-box;
  background-color: var(--base9-0);
  color: var(--base9-0-contrast);
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr 1fr;
  grid-template-rows: 1fr 1fr 1fr;
  gap: 1em;
  align-items: begin;
  margin-bottom: 1em;
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
    border: 1px solid;
    border-color: var(--base9-c0-p10);
    box-sizing: border-box;
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

.bar {
    margin-top: 1em;
    margin-bottom: 1em;
    display: flex;
    gap: 1em;
    flex-flow: row-reverse;
}
</style>
<script setup lang="ts">
import { Dice, Copy, SunnyOutline, MoonOutline } from '@vicons/ionicons5'
import { renderString } from 'base9-builder';
import * as _ from 'lodash';
import { computed, inject, reactive } from 'vue';
import { Palette } from '../helper2';
import type { List9 } from '../helpers';

const palette = inject<Palette>('palette')!;
const locked = reactive([true, true, true, false, false, false, false, false, false] as List9<boolean>);

const props = defineProps<{
    label?: boolean,
}>();
const LABELS = [
    'background', 'foreground', 'primary', 'secondary', 'tertiary', 'quaternary', 'quinary', 'senary', 'septenary'
] as const;

const labels = computed(() => LABELS.map(l => props.label ? l : ''));

function getStyle(i: number) {
    return {
        background: `var(--base9-${i})`,
        height: `${(12-i)*10}%`,
        color: `var(--base9-${i}-contrast)`,
        gridColumn: `${i-1} / span 1`,
    };
}
function generate() {
    const paletteStr = palette.hexs.map((x, i) => locked[i] ? x.value : '_').join('-');
    palette.palette.value = renderString(paletteStr, '{{PALETTE}}')
}

const toggleTemplate = "{{foreground.p100.hex}}-{{background.hex}}-{{c1.p100.hex}}-{{c2.p100.hex}}-{{c3.p100.hex}}-{{c4.p100.hex}}-{{c5.p100.hex}}-{{c6.p100.hex}}-{{c7.p100.hex}}"

function toggleDark() {
    palette.palette.value = renderString(palette.palette.value, toggleTemplate)
}
</script>

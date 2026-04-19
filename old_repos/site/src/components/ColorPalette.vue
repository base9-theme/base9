<template>
<div @keypress="handleKeyPress">
  <n-popover
    trigger="click"
    v-for="(color, i) in colors"
    v-bind:key="i"
    placement="right-start"
  >
    <template #trigger >
      <div
        className="slot"
        @pointerenter="state[i].hover = true"
        @pointerleave="state[i].hover = false"
        :style="{ backgroundColor: color.hex(), color: contrastColor(color).hex()}"
      >
        {{color.hex()}}
        <n-button
          :style="{verticalAlign: 'middle'}"
          size="small"
          quaternary
          circle
          @click.stop="() => toggleLock(i)"
          #icon
        >
          <lock-open-round
            v-if="state[i].hover && !state[i].locked"
            :color="contrastColor(colors[i])"
          />
          <lock-filled
            v-if="state[i].locked"
            :color="contrastColor(colors[i])"
          />
          <!-- <menu-round/> -->
        </n-button>
      </div>
    </template>
    <ColorPicker v-model:color="colors[i]" :otherColors="getOtherColors(i)"/>
  </n-popover>
  <n-button
    type="primary"
    quaternary
    size="large"
    @click="generate"
    :style="{width: '100%', marginTop: '5px'}"
  >
    Generate<br>
    (Spacebar)
  </n-button>
  <import/>
  <base-16-scheme-picker/>
</div>
</template>
<style scoped>
.slot {
  height: 50px;
  line-height: 50px;
  text-align: center;
  cursor: pointer;
}
</style>
<script setup lang="ts">
import {
  inject, computed, Ref, ref,
} from 'vue';
import Color from 'color';
import _ from 'lodash';
import { contrastColor } from '../helpers';
import type { ColorPalette } from '../helpers';
import ColorPicker from './ColorPicker.vue';
import { renderString } from 'base9-builder';
import { useRouter } from 'vue-router';
import { LockOpenRound, LockFilled } from '@vicons/material';

const router = useRouter();
const colors = inject('colors') as Ref<ColorPalette>;
const state = ref(_.times(9, i => ({hover: false, locked: false})));

function getOtherColors(i: number) {
  const rtn = colors.value.map(c => ({
    color: c,
    label: '',
  }));
  rtn.splice(i, 1);
  return rtn;
}

async function generate() {
  const palette = renderString(colors.value.map((c, i) => state.value[i].locked ? c.hex().substring(1) : "_").join("-"), "{{PALETTE}}");
  await router.push({
    query: { palette: palette },
  });
}

function toggleLock(i: number) {
  state.value[i].locked = !state.value[i].locked;
}



function handleKeyPress(e: KeyboardEvent) {
  if(e.code === 'Space') {
    // generate();
  }
}
</script>

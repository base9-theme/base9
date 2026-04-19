<template>
  <div>
    <ColorPicker v-model:color="colors[2]" :otherColors="colors.slice(3).map((c, i) => ({color: c, label: ''}))"></ColorPicker>
    <a-button>a</a-button>
    <h2>Absolute Colors</h2>
    <canvas id="canvas" width="500" height="500"></canvas>
  </div>
</template>
<style scoped>
</style>
<script setup lang="ts">

import {
  computed, defineComponent, inject, Ref, ref, watchEffect, watchPostEffect,
} from 'vue';
import Color from 'color';
import _ from 'lodash';
import {tmp} from '../debug';

// import Ansi from './Ansi.vue';

// import tmp from '..assets'

import type { ColorPalette } from '../helpers';

console.log(tmp);
const colors = inject('colors') as Ref<ColorPalette>;
watchPostEffect(() => {
  const absolute = [
    Color("#ff0000"),
    Color("#ffff00"),
    Color("#00ff00"),
    Color("#00ffff"),
    Color("#0000ff"),
    Color("#ff00ff"),
    Color("#000000"),
  ];
  const colors1 = colors.value.slice(2).map((c: Color) => {
    return Color.lab([c.l(), c.a(), c.b()]);
  })
  // console.log(colors.value[0].hex().substring(1)+"-"+colors.value[1].hex().substring(1)+"-"+colors1.map(c=>c.hex().substring(1)).join("-"));
  run(colors1, absolute);
})


function run(colors1: Color[], colors2: Color[]) {
  const canvas = document.getElementById("canvas") as HTMLCanvasElement;
  const ctx = canvas.getContext("2d")!;
  ctx.resetTransform();
  const unit = 200;

  //clear
  ctx.fillStyle = "#000000";
  ctx.fillRect(0,0,canvas.width, canvas.height);
  ctx.translate(canvas.width/2, canvas.height/2);

  //axis
  ctx.fillStyle = "#ffffff";
  ctx.strokeStyle = "#ffffff";
  ctx.fillRect(-1000, 0,2000,1);
  ctx.fillRect(0, -1000, 1,2000);
  ctx.beginPath();
  ctx.ellipse(0,0,unit,unit,0,0,Math.PI*2);
  ctx.closePath();
  ctx.stroke();
  for(let i = 0 ; i < colors1.length; i++) {
    const xy1 = colorToXy(colors1[i]);
    const xy2 = colorToXy(colors2[i]);
    ctx.strokeStyle = "#ffffff";
    ctx.beginPath();
    ctx.ellipse(xy1[0], xy1[1], 8, 8, 0,0,Math.PI*2);
    ctx.closePath();
    ctx.fillStyle = colors1[i].hex();
    ctx.fill();
    ctx.stroke();
    ctx.strokeStyle = "#000000";
    ctx.beginPath();
    ctx.ellipse(xy2[0], xy2[1], 8, 8, 0,0,Math.PI*2);
    ctx.closePath();
    ctx.fillStyle = colors2[i].hex();
    ctx.fill();
    ctx.stroke();
    ctx.strokeStyle = "#ffffff";
    ctx.beginPath();
    ctx.moveTo(xy1[0], xy1[1]);
    ctx.lineTo(xy2[0], xy2[1]);
    ctx.closePath();
    ctx.stroke();
  }
}

function colorToXy(color: Color): [number, number] {
  return [color.a(), color.b()]
}


</script>

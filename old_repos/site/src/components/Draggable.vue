
<template>
    <div ref="divRef" v-on:mousedown.left="handleMouseDown" >
        <slot/>
    </div>
</template>
<script setup lang="ts">
export interface DragEvent {x: number, y: number, type: 'begin'|'drag'|'end'};

import { on, off } from "evtd";
import _ from "lodash";
import { ref } from "vue";
const divRef = ref<HTMLDivElement|null>(null)
const initXY = ref({x: 0, y: 0});

const props = defineProps<{
    cb: (e: DragEvent) => void,
    clamp?: boolean,
    relative?: boolean,
}>();
function handleMouseDown(e: MouseEvent) {
    if (!divRef.value) return;
    on('mousemove', document, handleMouseMove);
    on('mouseup', document, handleMouseUp);
    handleMouseMove(e);
}

function getType(e: MouseEvent) {
    if(e.type === 'mousedown' && e.button === 0) {
        return 'begin';
    }
    if(e.type === 'mousemove') {
        return 'drag';
    }
    if(e.type === 'mouseup' && e.button === 0) {
        return 'end';
    }
    return undefined;
}

function handleMouseMove(e: MouseEvent) {
  const { width, left, height, top } = divRef.value!.getBoundingClientRect()
  const type = getType(e);
  
  let x = (e.clientX - left)/width;
  let y = (e.clientY - top)/height;
  if(!type) {
    return;
  }
  if(type === 'begin') {
    initXY.value = {x,y};
  }
  if(props.relative) {
    x -= initXY.value.x;
    y -= initXY.value.y;
  }
  if(props.clamp) {
    x = _.clamp(x, 0, 1);
    y = _.clamp(y, 0, 1);
  }
  props.cb({x, y, type});
}
function handleMouseUp(e: MouseEvent) {
    off('mousemove', document, handleMouseMove)
    off('mouseup', document, handleMouseUp)
    handleMouseMove(e);
}
</script>
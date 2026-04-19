
<template>
  <n-popover
    trigger="click"
    placement="top"
  >
    <template #trigger >
      <div :style="{ backgroundColor: color.hex(), color: contrastColor(color).hex() }"
        className="pick"
      >
        {{color.hex()}}
      </div>
    </template>
    <ColorPicker :color="color" @update:color="emits('update:color', $event)"/>
  </n-popover>
</template>
<style scoped>
.pick {
  padding: 0 0.3em;
    font-size: 4em;
    cursor: pointer;
}

</style>
<script setup lang="ts">
import Color from "color";
import { contrastColor } from "../helpers";

const props = defineProps<{
    color: Color,
}>();

const emits = defineEmits<{
  (e: 'update:color', c: Color): void,
}>();
</script>


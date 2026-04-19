<template>
<div class="container" >
  <div class="container2">
    <div class="title">
      {{props.theme.name}}
    </div>
    <div class="subtitle">
      {{props.theme.author}}
    </div>
    <div class="tags">
      <n-tag
        v-for="tag in theme.tags"
        v-bind:key="tag"
        type="primary"
        :bordered="false"
        size="small"
      >
        {{tag}}
      </n-tag>
    </div>
    <!-- <div class="actions">
      <n-button size="small">set</n-button>
      <n-button size="small">preview</n-button>
      <n-button size="small">export</n-button>
    </div> -->
  </div>
  <ColorPalette3 class="palette" :palette="theme.palette"></ColorPalette3>
</div>
</template>
<style scoped>
.container {
  border: 1px solid var(--base9-foreground-p50);
  background-color: var(--base9-foreground-p10);
  display: flex;
  flex-flow: column;
  border-radius: 0.5em;
  overflow: hidden;
}
.container2 {
  padding: 1em;
  display: flex;
  flex-flow: column;
  gap: 0.5em;
}
.title {
  font-size: 1.2em;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}
.subtitle {
  color: var(--base9-fg-p75);
  width: 100%;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}
.tags {
  display: flex;
  gap: 0.5em;
}
.actions {
  display: flex;
  gap: 0.5em;
}
.palette {
  box-sizing: border-box;
  padding: 1em;
  height: 12em;
  flex-grow: 1;
  border-top: 1px solid var(--base9-foreground-p50);
}
</style>
<script setup lang="ts">
import { Data } from "base9-builder";
import { computed, inject } from "vue";
import type { Ref } from "vue";
import type { Theme } from "../themes";
import {Palette} from '../helper2';

const palette = inject<Palette>('palette')!;
const props = defineProps<{theme: Theme}>();
const containerStyle = computed(() => ({
  borderColor: "#"+palette.colorData.value.foreground.p25.hex,
  backgroundColor: "#" + palette.colorData.value.foreground.p10.hex,
}))

</script>
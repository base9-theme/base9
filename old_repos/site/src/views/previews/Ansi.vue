<template>
<div class="list">
  <div
    class="cell"
    v-for="i in _.range(8)"
    v-bind:key="i"
  >
    {{ ansiLabel[i] }}
  </div>
  <div
    class="cell"
    :style="{ background: `var(--base9-ansi-c${HEX[i]})` }"
    v-for="(i) in _.range(16)"
    v-bind:key="i"
  >
  </div>
</div>
</template>
<style scoped>
.list {
  display: grid;
  grid-template-columns: repeat(8, 1fr);
}
.cell {
  height: 20px;
  text-align: center;
}

</style>
<script setup lang="ts">
import _ from 'lodash';
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
const HEX = '0123456789abcdef';
</script>

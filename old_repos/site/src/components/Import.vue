<template>
<n-form-item :validation-status="valid ? 'success' : 'error'" :show-label="false" :show-feedback="false">
  <n-input-group>
    <n-input :placeholder="placeholder" :value="input" @update:value="updateInput"/>
    <n-button @click="copy" #icon ><content-copy-sharp/></n-button>
  </n-input-group>
</n-form-item>
</template>

<script setup lang="ts">
import {
  ref, watch, computed, Ref, inject,
} from 'vue';
import _ from 'lodash';
import { ContentCopySharp } from '@vicons/material';
import { useMessage } from 'naive-ui';
import Color from 'color';
import type { ColorPalette } from '../helpers';
import { toPaletteString, PALETTE_REGEX } from '../helpers';

const message = useMessage();
const placeholder = '111111-222222-333333-444444-555555-666666-777777-888888-999999';

const colors = inject('colors') as Ref<ColorPalette>;
const input = ref(toPaletteString(colors.value));
const valid = computed(() => PALETTE_REGEX.test(input.value));

watch(colors, (colorsNew) => {
  input.value = toPaletteString(colorsNew);
});

function copy() {
  navigator.clipboard.writeText(input.value);
  message.success('Copied');
}
function updateInput(newInput: string) {
  input.value = newInput;
  if (PALETTE_REGEX.test(newInput)) {
    const colors2 = _.map(newInput.split('-'), (s) => Color(`#${s}`)) as ColorPalette;
    colors.value = colors2;
  }
}
</script>

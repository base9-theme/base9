<template>
<n-select
  :value="value"
  @update:value="updateValue"
  placeholder="Import from Base16"
  :render-label="renderLabel"
  filterable
  :options="options"/>
</template>
<script setup lang="ts">
import {
  h,
  inject, Ref, ref, VNode, watch,
} from 'vue';
import _ from 'lodash';
import { NIcon, useMessage, SelectOption as NSelectOption} from 'naive-ui';
// import SelectOption from './SelectOption.vue';
import Logo from './Logo.vue';
import compactTemplate from '../assets/templates/compact.svg.mustache';
import {
  getColorsFromBase16, getColors16FromSchemeObj, toPaletteString, render
} from '../helpers';
import type { ColorPalette } from '../helpers';

const message = useMessage();

const schemesRaw = import.meta.globEager('../assets/schemes-base16/*.yaml');

const options = _.entries(schemesRaw).map(([k, v], i) => {
  const key = ((/\.\.\/assets\/schemes-base16\/(.*)\.yaml/.exec(k) || [])[1]);
  const colors16 = getColors16FromSchemeObj(v);
  return {
    label: key,
    value: i,
    data: {
      name: key,
      colors16,
      colors: getColorsFromBase16(colors16),
    },
  };
});

// let tmp = ""
// _.entries(schemesRaw).map(([k, v], i) => {
//   const colors16 = getColors16FromSchemeObj(v);
//   const key = ((/\.\.\/assets\/schemes-base16\/(.*)\.yaml/.exec(k) || [])[1]);
//   tmp += key +"	"
//   _.times(16, i => {
//     tmp += colors16[i].red() + "	";
//     tmp += colors16[i].green() + "	";
//     tmp += colors16[i].blue() + "	";
//   });
//   tmp +="\n"
// })
// console.log(tmp)


const renderLabel = function (option: typeof options[number]) {
  return [
    h(
      NIcon,
      {
        style: {
          verticalAlign: 'middle',
          marginRight: '4px',
        },
        innerHTML: render(compactTemplate, option.data.colors),
      }
    ),
    option.label,
  ];
};
const value = ref<null|number>(null);
const colors = inject('colors') as Ref<ColorPalette>;

watch(colors, (newColors: ColorPalette) => {
  const valueRaw = value.value;
  if (valueRaw !== null
  && toPaletteString(newColors) !== toPaletteString(options[valueRaw].data.colors)) {
    value.value = null;
  }
});

function updateValue(newValue: number|null) {
  value.value = newValue;
  if (newValue !== null) {
    const option = options[newValue];
    colors.value = option.data.colors.slice() as ColorPalette;
    message.success(`Imported: ${option.label}`);
  }
}
</script>

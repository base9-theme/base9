<template>
<n-icon :size="size" v-html="content"/>
</template>
<style>

</style>
<script setup lang="ts">
import { computed, defineProps } from 'vue';
import { render } from '../helpers';
import type { ColorPalette } from '../helpers';
import { renderString } from 'base9-builder';
import logoTemplate from '../assets/templates/logo.svg.mustache';

const props = defineProps<{
    palette: string,
    size?: number|string,
}>();
const content = computed(() => renderString(props.palette, logoTemplate));
</script>

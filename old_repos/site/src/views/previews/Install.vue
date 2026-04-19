<template>
<n-input-group class="input-group">
<n-input :input-props="{spellCheck: 'false'}" :value="palette.palette.value" />
    <n-button type="primary" ghost @click="handleCopy">
        <n-icon><copy /></n-icon>
    </n-button>
    <router-link to="/install">
    <n-button type="primary">
    Install for specific apps
    </n-button>
    </router-link>
</n-input-group>
</template>
<style scoped>
.input-group {
    margin-bottom: 1em;
}
</style>
<script setup lang="ts">
import { Dice, Copy, SunnyOutline, MoonOutline } from '@vicons/ionicons5'
import { palette_key } from "../../helper2";
import { inject } from "vue";
import { useMessage } from 'naive-ui';
const message = useMessage();

const palette = inject(palette_key)!;

function handleCopy() {
  message.success('Palette Copied');
  navigator.clipboard.writeText(palette.palette.value);
}
</script>

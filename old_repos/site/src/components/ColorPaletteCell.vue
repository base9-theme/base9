import Color from "color";

<template>
<div
    @pointerenter="() => setHover(true)"
>{{hover}}</div>
</template>
<style scoped>

</style>
<script setup lang="ts">
import Color from "color";
import { ref } from "vue";

const props = defineProps<{
    color: Color,
    label: string,
}>();

const hover = ref(false);
function setHover(b: boolean) {
    hover.value = b;
}

</script>

<template>
<div v-html="content">
</div>
</template>
<style>

</style>
<script setup lang="ts">
import { marked } from 'marked';

const props = defineProps<{mdString: string}>()
const content = marked(props.mdString, { sanitize: true });
</script>

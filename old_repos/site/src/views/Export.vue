<template>
<Main>
<div class="container">
  <!-- <n-select v-model:value="value" :options="options" /> -->
  <n-select
    v-model:value="value"
    filterable
    placeholder="Select app"
    class="tag-filter"
    :options="options"
  />
  <!-- <n-code :code="content" language="cpp">
  </n-code> -->
  <template v-if="installation && config">
  <h1><a :href="config.app.link">{{config.app.name}}</a></h1>
  <h2>Installation</h2>
  <Markdown :source="beforeMd"></Markdown>
  <div v-if="installation.write_template">
    <p>Save file to <code>{{installation.destination}}</code></p>
    <n-button @click="downloadTemplate">Download</n-button>
  </div>
  <div v-if="installation.copy_template">
    <n-button @click="copyTheme">Copy Theme</n-button>
  </div>
  <div v-if="installation.copy_palette">
    <n-button @click="copyPalette">Copy palette</n-button>
  </div>
  <Markdown :source="afterMd"></Markdown>
  </template>

</div>
</Main>
</template>
<style>
.container code {
  background-color: var(--base9-foreground-p10);
}
.content {
  white-space: pre-wrap;
  font-family: monospace;
}
</style>

<script setup lang="ts">
import Markdown from 'vue3-markdown-it';
import _, { Dictionary } from 'lodash';
import {
  computed, inject, Ref, ref,
} from 'vue';
import yaml from 'js-yaml';
import { renderString } from 'base9-builder';
import fileDownload from 'js-file-download';
import type { ColorPalette } from '../helpers';
import type { Config } from '../../base9-templates/scripts/config';
import { palette_key } from '../helper2';
import { useMessage } from 'naive-ui';
const TEMPLATE_PREFIX = '../../base9-templates/templates/';
const message = useMessage();

// Cannot replace with prefix, requires literal.
const configRaw = import.meta.globEager('../../base9-templates/templates/*/config.yml');
const templateRaw = import.meta.globEager('../../base9-templates/templates/*/*.mustache');
const configObj: Dictionary<Config> = Object.fromEntries(_.entries(configRaw).map(([k, v]) => ([
  ((/\.\.\/\.\.\/base9-templates\/templates\/(.*)\/config.yml/.exec(k) || [])[1]),
  v.default as Config,
])));
// console.log(configObj);

const options = _.map(configObj, (v, k) => ({
  label: v.app.name,
  value: k,
}));

const value = ref(null);
const palette = inject(palette_key)!;

const config = computed(() => {
  if (!value.value) {
    return null;
  }
  return configObj[value.value];
});

const installation = computed(() => {
  if (!config.value) {
    return null;
  }
  return config.value.installation[0];
})

const PALETTE_PLACEHOLDER = "111111-222222-333333-444444-555555-666666-777777-888888-999999";

const beforeMd = computed(() => {
  return (installation.value?.before_md as string ?? '').replaceAll(PALETTE_PLACEHOLDER, palette.palette.value);
});
const afterMd = computed(() => {
  return (installation.value?.after_md as string ?? '').replaceAll(PALETTE_PLACEHOLDER, palette.palette.value);
});

function downloadTemplate() {
  const theme = renderString(palette.palette.value, templateRaw[TEMPLATE_PREFIX + installation.value?.template].default);
  const destination = installation.value?.destination as string;
  fileDownload(theme, destination.split('\/').pop()!);
}

function copyPalette() {
  navigator.clipboard.writeText(palette.palette.value);
  message.success('Palette Copied');
}

function copyTheme() {
  const theme = renderString(palette.palette.value, templateRaw[TEMPLATE_PREFIX + installation.value?.template].default);
  navigator.clipboard.writeText(theme);
  message.success('Theme Copied');
}

const content = computed(() => {
  if (!value.value) {
    return null;
  }
  const template = templateRaw[`../../base9-templates/templates/${configObj[value.value].installation[0].template}`].default
  return renderString(palette.palette.value, template);
});

</script>

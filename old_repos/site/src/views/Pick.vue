<template>
<Main>
<div class="menu">
  <n-select
    v-model:value="selectedTags"
    multiple
    filterable
    clearable
    placeholder="Filter by Tags"
    class="tag-filter"
    :options="tagOptions"
  />
  <n-input
    class="name-search"
    v-model:value="searchedName"
    type="text"
    clearable
    placeholder="Search by Name"
  />
  <n-select
    class="sort-by"
    v-model:value="sortBy"
    :options="sortOptions"
  />
</div>
<div class="themes">
  <PickCard
    v-for="(theme, i) in filteredThemes"
    v-bind:key="i"
    class="theme"
    :theme="theme"
    @click="selectTheme(theme)"
  >
  </PickCard>
</div>
</Main>
</template>
<style scoped>
.menu {
  display: flex;
  /* justify-content: center; */
  align-items: center;
  gap: 1em;
}
.tag-filter {
  flex-shrink: 1;
}
.name-search {
  flex-shrink: 1;
}
.sort-by {
  flex-shrink: 2;
}
.themes {
  margin-top: 1em;
  display: flex;
  flex-flow: row wrap;
  gap: 1em;
}
.theme {
  width: 23%;
  cursor: pointer;
}
</style>
<script setup lang="ts">
import {
  computed, defineComponent, h, inject, nextTick, Ref, ref, watch,
} from 'vue';
import Color from 'color';
import _ from 'lodash';
// import Ansi from './Ansi.vue';

// import tmp from '..assets'
import { toPaletteString } from '../helpers';
import * as base9 from 'base9-builder';
import { NTag } from 'naive-ui/lib/tag';
import { themes, TAGS } from '../themes/index';
// import type { AutoCompleteInst } from 'naive-ui'
import type { ColorPalette } from '../helpers';
import { Tag, Theme } from '../themes/index';
import { useRouter } from 'vue-router';

const router = useRouter();

const tagOptions = TAGS.map(t => ({label: t, value: t}))
const sortOptions = [
  {
    label: 'a-z',
    value: 'a-z',
  },
  {
    label: 'z-a',
    value: 'z-a',
  },
];

const selectedTags = ref([] as Tag[]);
const searchedName = ref("");
const inputValue = ref('');
const sortBy = ref('a-z');

const filteredThemes = computed(() => {
  const themes2 = themes.filter(theme =>
    selectedTags.value.every(niddle => theme.tags.includes(niddle)) &&
    theme.name.toLowerCase().includes(searchedName.value.toLowerCase())
  );
  return themes2;
  // return _.sortBy(themes2, theme => theme.author);
});

function selectTheme(theme: Theme) {
  router.push({
    path: '/preview',
    query: { palette: theme.palette },
  });
}

const renderTag = (tag: string, index: number) => {
        return h(
          NTag,
          {
            type: index < 3 ? 'success' : 'error',
            disabled: index > 3,
            // bordered: false,
            closable: true,
            onClose: () => {
              selectedTags.value.splice(index, 1)
            }
          },
          {
            default: () => tag
          }
        )
      }

const autoCompleteInstRef = ref<any | null>(null);
watch(autoCompleteInstRef, (value) => {
  if (value) nextTick(() => value.focus())
});

const previewListRows = ['0', '1', '2', '3', '4', '5', '6', '7'] as const;
type PreviewListRowsType = (typeof previewListRows)[number];
type x = `c${PreviewListRowsType}`;
const previewListColumns: [
  string,
  (c: PreviewListRowsType, colors: base9.Colors<string>) => string,
  (c: PreviewListRowsType, colors: base9.Colors<string>) => string,
][] = [
  ['p10 bg', (c, colors) => colors[`c${c}`].p10, (c, colors) => colors.foreground.p100],
  ['p25 bg', (c, colors) => colors[`c${c}`].p25, (c, colors) => colors.foreground.p100],
  ['p50 bg', (c, colors) => colors[`c${c}`].p50, (c, colors) => colors.foreground.p100],
  ['p75 inv.', (c, colors) => colors[`c${c}`].p75, (c, colors) => colors.background],
  ['p75', (c, colors) => colors.background, (c, colors) => colors[`c${c}`].p75],
  ['p100', (c, colors) => colors.background, (c, colors) => colors[`c${c}`].p100],
  ['p125', (c, colors) => colors.background, (c, colors) => colors[`c${c}`].p125],
];
const colors = inject('colors') as Ref<ColorPalette>;
const colorData = computed(() => base9.getColors(toPaletteString(colors.value)));
</script>

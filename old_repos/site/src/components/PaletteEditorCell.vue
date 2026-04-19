<template>
<n-popover
  trigger="click"
  placement="bottom"
>
    <template #trigger>
        <n-tooltip trigger="hover">
            <template #trigger>
                <div class="cell" @mouseenter="setHover(true)" @mouseleave="setHover(false)">
                        <n-icon class="lock" :class="{ hide: !(hover || locked[i]) }" @click="toggleLock">
                            <LockOpenOutline v-if="!locked[i]" />
                            <LockClosed v-if="locked[i]" />
                        </n-icon>
                    <!-- </n-button> -->
                </div>
            </template>
            {{LABELS[i]}}
        </n-tooltip>
    </template>
    <ColorPicker v-model:color="color" :otherColors="otherColors"/>
</n-popover>
</template>

<style scoped>
.cell {
    height: 100%;
    width: 100%;
    /* cursor: grab; */
    display: flex;
    align-items: center;
    justify-content: center;
}
.lock {
    cursor: pointer;
}

.hide {
    visibility: hidden;
}
</style>
<script setup lang="ts">
import { inject, computed, ref } from 'vue';
import { Palette } from '../helper2';
import { LockOpenOutline, LockClosed } from '@vicons/ionicons5';

const props = defineProps<{
    i: number
    locked: boolean[];
}>();
const hover = ref(false);

const palette = inject<Palette>('palette')!;
const color = palette.colors[props.i];
const LABELS = [
    'background', 'foreground', 'primary', 'secondary', 'tertiary', 'quaternary', 'quinary', 'senary', 'septenary'
] as const;

const otherColors = computed(() => {
  const rtn = palette.colors.map(c => ({
    color: c.value,
    label: '',
  }));
  rtn.splice(props.i, 1);
  return rtn;
})

function toggleLock(e: PointerEvent) {
    e.stopPropagation();
    props.locked[props.i] = !props.locked[props.i];
}

function setHover(b: boolean) {
    hover.value = b;
}
</script>

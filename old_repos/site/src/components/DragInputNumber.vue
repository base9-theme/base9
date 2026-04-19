<template>
<n-input :value="localValue" @update:value="updateValue" size="small" >
    <template #suffix>
        <Draggable :style="{display: 'inline-flex'}" :relative="true" :cb="handleDrag">
            <n-icon :style="{cursor: 'ns-resize'}" color="#000000"><swap-vert-round/></n-icon>
        </Draggable>
    </template>
</n-input>
</template>
<script setup lang="ts">
import { SwapVertRound } from '@vicons/material';
import _ from 'lodash';
import { ref, watch } from 'vue';
import type { DragEvent } from './Draggable.vue';
const props = defineProps<{
    min: number,
    max: number,
    value: number,
}>();

const emit = defineEmits(['update:value']);
const localValue = ref(String(props.value));

function toString(n: number): string {
    return n.toFixed(0);
}

function updateValue(s: string) {
    localValue.value = s;
    if(isNaN(Number(localValue.value))) {
        return;
    }
    emit('update:value', Number(localValue.value));
}

watch(() => props.value, () => {
    localValue.value = toString(props.value);
});

const K = 10;
const dragged = ref(0);
function handleDrag(e: DragEvent) {
    if(e.type === 'begin') {
        dragged.value = props.value;
    }

    const value = _.clamp(- e.y / K * (props.max - props.min) + dragged.value, props.min, props.max);
    
    localValue.value = toString(value);
    if(e.type === 'end') {
        emit('update:value', value);
    }
}
</script>

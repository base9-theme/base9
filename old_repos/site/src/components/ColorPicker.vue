<template>
<div className="container">
        <Draggable
            :style="{
                position: 'relative',
                height: '250px',
                backgroundSize: '100% 100%',
                backgroundImage: `url(${canvasUrl})`
            }"
            :cb="handleDrag"
            :clamp="true"
        >
            <span
                className="color-circle color-circle-selected"
                :style="circleStyle(setting.tmpColor ?? color)"
            ></span>
            <template v-if="setting.flags.includes('compare')">
                <span
                    v-for="(c, i) in props.otherColors" v-bind:key="i"
                    className="color-circle"
                    :style="circleStyle(c.color)"
                >{{c.label}}</span>
            </template>
        </Draggable>
        <Draggable
            :style="{
                position: 'relative',
                height: '10px',
                backgroundSize: '100%',
                backgroundImage: `url(${canvasUrl2})`
            }"
            :cb="handleDrag2"
            :clamp="true"
        >
            <span
                className="color-circle color-circle-selected"
                :style="circleStyle2(setting.tmpColor ?? color)"
            ></span>
            <template v-if="setting.flags.includes('compare')">
                <template v-for="(c, i) in otherColors" v-bind:key="i">
                    <span
                        className="color-circle"
                        :style="circleStyle2(c.color)"
                    ></span>
                </template>
            </template>
        </Draggable>
    <div :style="{display: 'flex'}">
        <n-button v-on:click="nextColorSpace" size="small" quaternary :style="{width: '50px', marginRight: '5px'}">{{colorSpace.label}}</n-button>
        <DragInputNumber :min="colorSpace.ranges[0].min" :max="colorSpace.ranges[0].max" :value="components[0]" @update:value="updateComponents[0]"/>
        <DragInputNumber :min="colorSpace.ranges[1].min" :max="colorSpace.ranges[1].max" :value="components[1]" @update:value="updateComponents[1]"/>
        <DragInputNumber :min="colorSpace.ranges[2].min" :max="colorSpace.ranges[2].max" :value="components[2]" @update:value="updateComponents[2]"/>
    </div>
    <div :style="{display: 'flex'}">
        <n-popselect v-model:value="setting.flags" multiple :options="POP_SELECT_OPTIONS">
            <n-button size="small" quaternary :style="{width: '50px', marginRight: '5px'}" #icon><menu-round/></n-button>
        </n-popselect>
        <n-input v-model:value="hex" @focusout="hexFocusout" size="small" >
            <template #suffix>
                <n-icon :style="{cursor: 'pointer'}" @click="hexCopy" color="#000000"><content-copy-sharp/></n-icon>
            </template>
        </n-input>
    </div>
</div>
</template>
<style scoped>
.container {
    width: 250px;
}
.container > div {
    display: flex;
    margin-bottom: 5px;
    width: 250px;
}
.color-circle {
    transform: translate(-50%, -50%);
    position: absolute;
    user-select: none;
    text-align: center;
    width: 6px;
    height: 6px;
    border: 1px solid #fff;
    border-radius: 3px;
}
.color-circle-selected {
    z-index: 10;
    width: 12px;
    height: 12px;
    border: 2px solid #fff;
    border-radius: 6px;
}
</style>
<script setup lang="ts">
import { SwapVertRound, ContentCopySharp, MenuRound } from '@vicons/material';
import Color from "color";
import _ from "lodash";
import * as convert from 'color-convert';
import { computed, CSSProperties, Ref, ref, watch, watchPostEffect } from "vue";
import { ColorPalette } from "../helpers";
import { useMessage } from 'naive-ui';
(window as any).convert = convert;
(window as any).Color = Color;

type Number3 = [number, number, number];

type ColorSpace = {
    readonly label: string,
    readonly componentLabels: [string, string, string],
    readonly ranges: [{min: number, max: number},{min: number, max: number},  {min: number, max: number}],
    toComponents: (c: Color) => Number3,
    fromComponents: (components: Number3) => Color,
    preview: {
        xyzToColor: (xyz: Number3) => Color,
        colorToXyz: (c: Color) => Number3,
    }
}

const POP_SELECT_OPTIONS = [
    { label: "compare", value: "compare" },
    { label: "region", value: "region" },
] as const;

type FLAG = (typeof POP_SELECT_OPTIONS)[number]['value'];

const COLOR_SPACES = [
    {
        label: "RGB",
        componentLabels: ['R', 'G', 'B'],
        ranges: [{min: 0, max: 255}, {min: 0, max: 255},  {min: 0, max: 255}],
        toComponents: (c: Color) => {
            return c.rgb().array() as Number3;
        },
        fromComponents: (components: Number3) => {
            return Color.rgb(components);
        },
        preview: {
            xyzToColor(xyz: Number3): Color {
                const comp = [xyz[2]*360, xyz[0]*100, xyz[1]*100];
                return Color.hsv(comp);
            },
            colorToXyz(c: Color) {
                let [h,s,v] = c.hsv().array();
                return [s/100, v/100, h/360];
            }
        }
    } as ColorSpace,
    {
        label: "Lab",
        componentLabels: ['L', 'a', 'b'],
        ranges: [{min: 0, max: 100}, {min: -128, max: 127}, {min: -128, max: 127}],
        toComponents: (c: Color) => {
            return c.lab().array() as Number3;
        },
        fromComponents: (components: Number3) => {
            return Color.lab(components);
        },
        preview: {
            xyzToColor(xyz: Number3): Color {
                const comp = [xyz[1]*100, xyz[0]*133, xyz[2]*360];
                return Color.lch(comp);
            },
            colorToXyz(color: Color) {
                let [l,c,h] = color.lch().array();
                return [c/133, l/100, h/360];
            }
        }
    } as ColorSpace,
    {
        label: "Lch",
        componentLabels: ['L', 'c', 'h'],
        ranges: [{min: 0, max: 100}, {min: 0, max: 133}, {min: 0, max: 360}],
        toComponents: (c: Color) => {
            return c.lch().array() as Number3;
        },
        fromComponents: (components: Number3) => {
            return Color.lch(components);
        },
        preview: {
            xyzToColor(xyz: Number3): Color {
                const comp = [xyz[2]*100, xyz[0]*256-128, xyz[1]*256-128];
                return Color.lab(comp);
            },
            colorToXyz(c: Color) {
                let [l,a,b] = c.lab().array();
                return [a/256 + 0.5, b/256 + 0.5, l/100];
            }
        }
    } as ColorSpace,
] as const;

function nextColorSpace() {
    setting.value.colorSpaceIndex = (setting.value.colorSpaceIndex + 1) % COLOR_SPACES.length;
}

const props = withDefaults(defineProps<{
    color: Color,
    otherColors?: { color: Color, label: string }[],
}>(), {
    otherColors: () => [],
});

const emit = defineEmits(["update:color"]);

const setting = ref({
    tmpColor: undefined as undefined|Color,
    colorSpaceIndex: 2,
    flags: ['compare', 'region'] as FLAG[],
    tmpNumber: 0,
});

const colorSpace = computed(() => COLOR_SPACES[setting.value.colorSpaceIndex]);
const components = computed(() => colorSpace.value.toComponents(setting.value.tmpColor ?? props.color));
const updateComponents = _.times(3, (i) => ((n: number) => {
    const cs = components.value.slice() as Number3;
    cs[i] = n;
    emit('update:color', colorSpace.value.fromComponents(cs))
}));

const hex = ref(props.color.hex());
const message = useMessage();
function hexFocusout() {
    console.log(hex.value);
    if(hex.value.length === 0) {
        return;
    }
    try {
        emit('update:color', Color(hex.value));
    } catch(e) {

    }
}
function hexCopy() {
  navigator.clipboard.writeText(hex.value);
  message.success('Copied');
}
watch(props, () => {
    hex.value = props.color.hex();
})

type ColorSpaceName = keyof typeof COLOR_SPACES;

const handleDrag = (e: DragEvent) => {
    const oldXyz = colorSpace.value.preview.colorToXyz(props.color);
    let newColor = colorSpace.value.preview.xyzToColor([e.x, e.y, oldXyz[2]]);

    if(e.type === 'end') {
        emit('update:color', newColor);
        setting.value.tmpColor = undefined;
    } else {
        setting.value.tmpColor = newColor;
    }
}

const handleDrag2 = (e: DragEvent) => {
    const oldXyz = colorSpace.value.preview.colorToXyz(props.color);
    let newColor = colorSpace.value.preview.xyzToColor([oldXyz[0], oldXyz[1], e.x]);

    if(e.type === 'end') {
        emit('update:color', newColor);
        setting.value.tmpColor = undefined;
    } else {
        setting.value.tmpColor = newColor;
    }
}

const SAMPLE_SIZE = 100;
const w = SAMPLE_SIZE;
const h = SAMPLE_SIZE;

function paintPixel(img: ImageData, x: number, y: number, c: Color) {
    const index = 4 * (y*w + x);
    img.data[index + 0] = c.red();
    img.data[index + 1] = c.green();
    img.data[index + 2] = c.blue();
    img.data[index + 3] = 255;
}

function circleStyle(c: Color): CSSProperties {
    return {
        top: `${colorSpace.value.preview.colorToXyz(c)[1]*100}%`,
        left: `${colorSpace.value.preview.colorToXyz(c)[0]*100}%`,
        background: c.hex(),
    }
}

function circleStyle2(c: Color): CSSProperties {
    return {
        top: `50%`,
        left: `${colorSpace.value.preview.colorToXyz(c)[2]*100}%`,
        background: c.hex(),
    }
}

function xyzDistance(xyz1: Number3, xyz2: Number3) {
    return Math.hypot(xyz1[0] - xyz2[0], xyz1[1] - xyz2[1], xyz1[2] - xyz2[2]);
}


const canvasUrl = computed(() => {
    const showRegion = setting.value.flags.includes('region');
    const canvas = document.createElement('canvas');
    canvas.width = SAMPLE_SIZE;
    canvas.height = SAMPLE_SIZE;
    const ctx = canvas.getContext("2d")!;
    const img = ctx.createImageData(w,h);
    const oldXyz = colorSpace.value.preview.colorToXyz(props.color);
    for(let xi = 0; xi < w; xi++) {
        for(let yi = 0; yi < h; yi++) {
            const xyz = [xi/w, yi/h, oldXyz[2]] as Number3;
            let color = colorSpace.value.preview.xyzToColor(xyz).rgb();
            const testXyz = colorSpace.value.preview.colorToXyz(color);
            if(showRegion && xyzDistance(xyz, testXyz) > 0.01) {
                // color = color.darken(0.1);
                color = Color.rgb(_.map(color.rgb().array(), x => x*0.7))
            }
            paintPixel(img, xi, yi, color);
        }
    }
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.putImageData(img, 0, 0);
    return canvas.toDataURL();
});
const canvasUrl2 = computed(() => {
    const showRegion = setting.value.flags.includes('region');
    const canvas = document.createElement('canvas');
    canvas.width = SAMPLE_SIZE;
    canvas.height = 1;
    const ctx = canvas.getContext("2d")!;
    const img = ctx.createImageData(w,h);
    const oldXyz = colorSpace.value.preview.colorToXyz(props.color);
    for(let xi = 0; xi < w; xi++) {
        const xyz = [oldXyz[0], oldXyz[1], xi/w] as Number3;
        let color = colorSpace.value.preview.xyzToColor(xyz).rgb();
        const testXyz = colorSpace.value.preview.colorToXyz(color);
        if(showRegion && xyzDistance(xyz, testXyz) > 0.01) {
            color = Color.rgb(_.map(color.rgb().array(), x => x*0.7))
        }
        paintPixel(img, xi, 0, color);
    }
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.putImageData(img, 0, 0);
    return canvas.toDataURL();
});
</script>
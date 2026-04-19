import { computed, ref } from "vue";
import type { Ref, WritableComputedRef, InjectionKey } from "vue";
import { I9, N9 } from "./helpers";
import Color from "color";
import _ from "lodash";
import { getData } from "base9-builder";

export const DEFAULT_PALETTE = '16161D-E0E5C8-CEAA7A-A6B2C1-CBA8B3-8AB9B2-DBA595-93BC95-B0B47A';
export const PALETTE_REGEX = /^([0-9a-fA-F]{6}-){8}[0-9a-fA-F]{6}$/;
export const palette_key = Symbol() as InjectionKey<Palette>

export class Palette {
    palette: Ref<string>;
    hexs = _.times(N9, i => computed({
      get: () => this.palette.value.substring(i*7, i*7+6),
      set: (c: string) => {
        const start = i*7;
        const end = i*7+6;
        this.palette.value =
          this.palette.value.substring(0,start) +
          c +
          this.palette.value.substring(end);
      }
    }));
    colors = _.times(N9, i => computed({
      get: () => Color("#"+this.hexs[i].value),
      set: (c: Color) => {
        this.hexs[i].value = c.hex().substring(1);
      }
    }));
    colorData = computed(() => getData(this.palette.value));
    dark = computed(() => this.colors[0].value.l() < this.colors[1].value.l());
    constructor(palette: string) {
      this.palette = ref(palette);
    }
}

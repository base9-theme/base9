import { Formatted, renderString } from 'base9-builder';
import Color from 'color';
import _, { Dictionary } from 'lodash';

export const N9 = 9 as const;
export type List9<T> = [T,T,T,T,T,T,T,T,T];
export type I9 = 0|1|2|3|4|5|6|7|8;
export type ShadesFormatted = {
  p10: Formatted,
  p25: Formatted,
  p50: Formatted,
  p75: Formatted,
  p100: Formatted,
  p125: Formatted,
}
export const DEFAULT_PALETTE = '16161D-E0E5C8-CEAA7A-A6B2C1-CBA8B3-93BC95-DBA595-8AB9B2-B0B47A';

const base24Digits = [
  'base00',
  'base01',
  'base02',
  'base03',
  'base04',
  'base05',
  'base06',
  'base07',
  'base08',
  'base09',
  'base0A',
  'base0B',
  'base0C',
  'base0D',
  'base0E',
  'base0F',
];
export function getCssVariableName(i: number) {
  return `--base9-${i}`;
}

export function getCssVariableContrastName(i: number) {
  return `--base9-${i}-contrast`;
}

export function toPaletteString(color9: Color[]) {
  return _.map(color9, (c) => c.hex().toString().substring(1)).join('-');
}

export function render(template: string, color9: Color[]) {
  return renderString(toPaletteString(color9), template);
}

export type ColorPalette = [
  Color, Color, Color, Color, Color, Color, Color, Color, Color,
];

export const PALETTE_REGEX = /^([0-9a-fA-F]{6}-){8}[0-9a-fA-F]{6}$/;


export function getColors16FromSchemeObj(schemeObj: Dictionary<any>) {
  const colors16 = _.map(base24Digits, (v, k) => {
    const colorStr = schemeObj.default[v];
    if (colorStr[0] === '#') {
      return Color(colorStr);
    }
    return Color(`#${colorStr}`);
  });
  return colors16;
}

export function contrastColor(c: Color) {
  return c.l() < 50 ? Color("#ffffff") : Color("#000000");
}

export function getColorsFromBase16(colors: Color[]): ColorPalette {
  return [
    colors[0],
    colors[5],
    colors[13],
    colors[12],
    colors[11],
    colors[8],
    colors[9],
    colors[10],
    colors[14],
  ];
}


export function getColor(palette: string, i: I9): Color {
  return Color("#"+palette.substring(i*7, i*7+6));
}
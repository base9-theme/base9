import Color from 'color';
import _ from 'lodash';
import { ColorPalette, getColorsFromBase16, getColors16FromSchemeObj, toPaletteString, DEFAULT_PALETTE } from '../helpers';
import community from './community.json';
import thirdParty from './third-party.yml';

const schemesRaw = import.meta.globEager('../assets/schemes-base16/*.yaml');

const base16Themes: BasicTheme[] = _.entries(schemesRaw).map(([k, v], i) => {
  const key = ((/\.\.\/assets\/schemes-base16\/(.*)\.yaml/.exec(k) || [])[1]);
  const value = v.default;
  const colors16 = getColors16FromSchemeObj(v);
  return {
    name: `Base16 ${value.scheme}`,
    author: value.author,
    palette: toPaletteString(getColorsFromBase16(colors16)),
  };
});
type BasicTheme = {
    name: string,
    author: string,
    palette: string,
    tags?: Tag[],
};

export const TAGS = [
  'dark',
  'light',
  'official',
  'third-party',
  'base16',
  'community',
] as const;
export type TypeTag = 'official'|'third-party' | 'base16' | 'community';
export type Tag = typeof TAGS[number];
export type Theme = BasicTheme & {
    tags: Tag[],
}

const defaultTheme: BasicTheme = {
    name: 'Default',
    author: 'base9',
    palette: DEFAULT_PALETTE,
}

export const themes = [
    toTheme(defaultTheme, 'official'),
    ..._.map(thirdParty, x => toTheme(x, 'third-party')),
    ..._.map(community, x => toTheme(x, 'community')),
    ..._.map(base16Themes, x => toTheme(x, 'base16')),
];

function toTheme(json: BasicTheme, type: TypeTag) {
    const tags: Tag[] = json.tags?.slice() || [];
    tags.push(type);
    const colors = _.map(json.palette.split('-'), (s) => Color(`#${s}`)) as ColorPalette;
    if(colors[0].l() < colors[1].l()) {
        tags.push('dark');
    } else {
        tags.push('light');
    }
    return {
        name: json.name,
        author: json.author,
        palette: json.palette,
        tags: tags,
    }
}

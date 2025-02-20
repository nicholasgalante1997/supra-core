import type { ThemeColor } from './themes';

const jsdelivr = 'https://cdn.jsdelivr.net/npm/@picocss/pico@2/css';
const unpkg = 'https://unpkg.com/@picocss/pico@2.0.6/css';

const classlessReplacer = (classless?: boolean) => (classless ? 'classless.' : '');

const themeReplacer = (theme: ThemeColor) => (theme === 'default' ? '' : `${theme}.`);

export const getThemeURI = (theme: ThemeColor, prefersClassless = false, cdn: 'jsdelivr' | 'unpkg' | string = 'jsdelivr') =>
  `${cdn === 'unpkg' ? unpkg : cdn === 'jsdelivr' ? jsdelivr : cdn}/pico.${classlessReplacer(prefersClassless)}${themeReplacer(theme)}min.css`;

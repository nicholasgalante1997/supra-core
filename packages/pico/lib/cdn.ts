import type { ThemeColor } from './themes';

const jsdelivr = 'https://cdn.jsdelivr.net/npm/@picocss/pico@2/css';
const unpkg = 'https://unpkg.com/@picocss/pico@2.0.6/css';

const classlessReplacer = (classless?: boolean) =>
  classless ? 'classless.' : '';

const themeReplacer = (theme: ThemeColor) =>
  theme === 'default' ? '' : `${theme}.`;

export const getThemeURI = (
  theme: ThemeColor,
  prefersClassless = false,
  cdn?: 'jsdelivr' | 'unpkg'
) =>
  `${cdn === 'unpkg' ? unpkg : jsdelivr}/pico.${classlessReplacer(prefersClassless)}${themeReplacer(theme)}min.css`;

/**
 * SECTION tests
 * - We need to write tests for each theme URL
 * !SECTION tests
 */

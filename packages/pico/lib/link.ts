import { getThemeURI } from './cdn';
import type { ThemeColor } from './themes';

export const LINK_ID = 'supra-pico-css-link';

type CreatePicoLinkOptions = {
  source?: string;
  theme?: ThemeColor;
  prefersClassless?: boolean;
  id?: string;
};

export function createLinkFromLinkOptions(
  options: CreatePicoLinkOptions
): HTMLLinkElement | null {
  if (typeof document === 'undefined') return null;

  const link = document.createElement('link');

  link.rel = 'stylesheet';
  link.dataset.supraIntent = 'pico';
  link.id = options.id || LINK_ID;

  const source = options.source || 'jsdelivr';
  const theme = options.theme || 'default';
  const prefersClassless = options.prefersClassless || false;

  switch (source) {
    case 'jsdelivr':
      link.href = getThemeURI(theme, prefersClassless, 'jsdelivr');
      break;
    case 'unpkg':
      link.href = getThemeURI(theme, prefersClassless, 'unpkg');
      break;
    default:
      if (source) link.href = source;
      break;
  }

  return link;
}

export function deleteLink() {
  if (typeof document === 'undefined') return;
  const link = document.getElementById(LINK_ID);
  if (link) {
    link.remove();
  }
}

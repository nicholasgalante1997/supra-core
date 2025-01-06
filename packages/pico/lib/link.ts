import { getThemeURI } from './cdn';
import type { ThemeColor } from './themes';

export const LINK_ID = 'supra-pico-css-link';
export const PREFETCH_ID = 'supra-pico-css-prefetch';

type CreatePicoLinkOptions = {
  htmlAttributes?: Record<string, any>;
  picoOptions?: {
    classless?: boolean;
    theme?: ThemeColor;
    cdn?: string;
  }
};

export function getDefaultLinkOptions(type?: 'preload' | 'stylesheet', mergeOptions: Record<string, any> = {}) {
  if (type === 'preload') {
    return {
      rel: 'preload',
      as: 'style',
      'data-supra-pico-id': PREFETCH_ID,
      ...mergeOptions
    };
  }

  return {
    rel: 'stylesheet',
    'data-supra-pico-id': LINK_ID,
    ...mergeOptions
  }
}

export function createLinkFromLinkOptions(options: CreatePicoLinkOptions): HTMLLinkElement | null {
  if (typeof document === 'undefined') return null;
  const link = document.createElement('link');

  if (options?.htmlAttributes) {
    for (const [attr, value] of Object.entries(options?.htmlAttributes)) {
      link.setAttribute(attr, value);
    }
  }

  const source = options?.picoOptions?.cdn || 'jsdelivr';
  const theme = options?.picoOptions?.theme || 'default';

  switch (source) {
    case 'jsdelivr':
      link.href = getThemeURI(theme, !!options?.picoOptions?.classless, 'jsdelivr');
      break;
    case 'unpkg':
      link.href = getThemeURI(theme, !!options?.picoOptions?.classless, 'unpkg');
      break;
    default:
      if (source) link.href = source;
      else link.href = getThemeURI(theme, !!options?.picoOptions?.classless);
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

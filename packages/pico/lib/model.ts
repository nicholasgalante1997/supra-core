import { setupOnKeyDownEventHandler } from './hotkeys';
import { createLinkFromLinkOptions, deleteLink } from './link';
import { prefersDarkMode } from './media';
import { mountThemeModeDataAttributeToNode } from './mode';
import { getDefaultDocumentRoot } from './root';
import { type ThemeColor } from './themes';
import createDefaultUi, { mountElementToRenderTarget } from './ui';

export type SupraPicoContructorOptions = {
  onInit?: () => void;
  onRender?: (ui: HTMLElement, root?: HTMLElement) => void;
  theme?: ThemeColor | 'default';
  skipInit?: boolean;
  skipRender?: boolean;
  source?: 'jsdelivr' | 'unpkg' | string;
  prefersClassless?: boolean;
  disableHotKeys?: boolean;
  ui?: HTMLElement;
  mode?: 'light' | 'dark';
  root?: HTMLElement | null;
};

const defaults: SupraPicoContructorOptions = {
  theme: 'default',
  mode: prefersDarkMode() ? 'dark' : 'light',

  skipInit: false,
  skipRender: false,

  source: 'jsdelivr',
  prefersClassless: false,

  ui: createDefaultUi(),
  root: getDefaultDocumentRoot(),

  disableHotKeys: false,

  onRender: (ui) => {
    mountElementToRenderTarget(ui, defaults.root!);
  },
  onInit: () => {
    console.log('[@supra/pico] initialized!');
  }
};

class SupraPico {
  private onInit?: () => void;
  private onRender?: (ui: HTMLElement) => void;
  private theme?: ThemeColor | 'default';
  private skipInit?: boolean;
  private skipRender?: boolean;
  private source?: 'jsdelivr' | 'unpkg' | string;
  private prefersClassless?: boolean;
  private ui?: HTMLElement;
  private mode?: 'light' | 'dark';
  private root?: HTMLElement | null;
  private disableHotKeys?: boolean;

  constructor(options: SupraPicoContructorOptions = {}) {
    options = this.mergePartialOptionsWithDefaultOptions(options);

    this.onInit = options.onInit;
    this.onRender = options.onRender;

    this.theme = options.theme;
    this.mode = options.mode;

    this.skipInit = options.skipInit;
    this.skipRender = options.skipRender;

    this.source = options.source;
    this.prefersClassless = options.prefersClassless;
    this.ui = options.ui;
    this.root = options.root;
    this.disableHotKeys = options.disableHotKeys;
  }

  init() {
    if (this.skipInit) return;
    this.mountLinkToDOM();
    this.setupInitialThemeMode();
    if (!this.disableHotKeys) this.setupHotKeys();
    if (this.onInit) this.onInit();
  }

  render() {
    if (typeof document === 'undefined') return;
    if (this.skipRender || !this.onRender) return;
    this.onRender(this.ui || createDefaultUi()!);
  }

  public updateTheme(theme: ThemeColor) {
    deleteLink();
    this.theme = theme;
    this.mountLinkToDOM();
  }

  public updateMode(mode: 'light' | 'dark') {
    this.mode = mode;
    this.updateThemeMode();
  }

  public unmount() {
    deleteLink();
    if (this.ui) this.ui.remove();
  }

  getTheme() {
    return this.theme;
  }

  getMode() {
    return this.mode;
  }

  private setupHotKeys() {
    if (this.disableHotKeys) return;
    if (typeof document === 'undefined') return;
    setupOnKeyDownEventHandler(this);
  }

  private mountLinkToDOM() {
    const link = this.createLink();
    if (link && typeof document !== 'undefined') {
      document.head.appendChild(link);
    }
  }

  private setupInitialThemeMode() {
    mountThemeModeDataAttributeToNode(this.root!, this.mode!);
  }

  private updateThemeMode() {
    mountThemeModeDataAttributeToNode(this.root!, this.mode!);
  }

  private createLink(): HTMLLinkElement | null {
    return createLinkFromLinkOptions({
      prefersClassless: this.prefersClassless,
      source: this.source,
      theme: this.theme
    });
  }

  private mergePartialOptionsWithDefaultOptions(
    options: SupraPicoContructorOptions = {}
  ) {
    return {
      ...defaults,
      ...options
    };
  }
}

export default SupraPico;

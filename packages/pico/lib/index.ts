import SupraPico, { type SupraPicoContructorOptions } from './model.js';
import themes from './themes.js';

Object.defineProperty(SupraPico, 'themes', {
  get: () => themes,
  value: themes,
  writable: false
});

function pico(options?: Partial<SupraPicoContructorOptions>) {
  const instance = new SupraPico(options);

  const init = () => {
    instance.init();
  }

  const render = () => {
    instance.render();
  }

  const unmount = () => {
    instance.unmount();
  }

  return {
    _pico: instance,
    init,
    render,
    unmount
  };
}

export { SupraPico, themes, pico };

export default pico;

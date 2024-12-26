import SupraPico, { type SupraPicoContructorOptions } from './model';
import themes from './themes';

Object.defineProperty(SupraPico, 'themes', {
  get: () => themes,
  value: themes,
  writable: false
});

interface PicoX {
  init(): void;
  render(): void;
  unmount(): void;
}

function pico(options?: Partial<SupraPicoContructorOptions>): PicoX {
  const instance = new SupraPico(options);

  const init = () => {
    instance.init();
  };

  const render = () => {
    instance.render();
  };

  const unmount = () => {
    instance.unmount();
  };

  const $pico = {} as PicoX;

  Object.defineProperty($pico, 'init', {
    value: init,
    writable: false,
    enumerable: true
  });

  Object.defineProperty($pico, 'render', {
    value: render,
    writable: false,
    enumerable: true
  });

  Object.defineProperty($pico, 'unmount', {
    value: unmount,
    writable: false,
    enumerable: true
  });

  Object.defineProperty($pico, '__root_instance__', {
    value: instance,
    writable: false,
    enumerable: false
  });

  return $pico as PicoX;
}

export { SupraPico, themes, pico };

export default pico;

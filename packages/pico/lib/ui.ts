function createDefaultUi(): HTMLElement | undefined {
  if (typeof document === 'undefined') {
    return undefined;
  }

  const ui = document.createElement('div');
  ui.classList.add('supra-pico', 'root');

  return ui;
}

export function mountElementToRenderTarget(element: HTMLElement, target: HTMLElement) {
  target.appendChild(element);
}

export default createDefaultUi;

export const modes = ['dark', 'light'] as const;

export function mountThemeModeDataAttributeToNode(node: HTMLElement, mode: (typeof modes)[number]) {
  node.dataset.theme = mode;
}

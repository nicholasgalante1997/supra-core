export function getDefaultDocumentRoot(): HTMLElement | null {
  if (typeof document === 'undefined') return null;
  return document.querySelector(':root') || document.body;
}

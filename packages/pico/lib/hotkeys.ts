import themeColors from './themes';
import SupraPico from './model';

export function setupOnKeyDownEventHandler(pico: SupraPico) {
  if (typeof document !== 'undefined') {
    function onKeyDownEventHandler(event: KeyboardEvent) {
      const currentThemeIndex = themeColors.indexOf(pico.getTheme()!);

      if (currentThemeIndex === -1) {
        console.warn('[@supra/pico] Could not find the current theme %s', pico.getTheme());
        return;
      }

      const isCommandKeyIntentfulPress = event.shiftKey && event.ctrlKey;
      const commandIntentKey = event.key;

      if (isCommandKeyIntentfulPress && commandIntentKey === 'ArrowRight') {
        const nextThemeIndex = (currentThemeIndex + 1) % themeColors.length;
        const nextTheme = themeColors[nextThemeIndex];
        pico.updateTheme(nextTheme);
        return;
      }

      if (isCommandKeyIntentfulPress && commandIntentKey === 'ArrowLeft') {
        const lastThemeIndex = currentThemeIndex === 0 ? themeColors.length - 1 : currentThemeIndex - 1;
        const lastTheme = themeColors[lastThemeIndex];
        pico.updateTheme(lastTheme);
        return;
      }

      if (isCommandKeyIntentfulPress && commandIntentKey === 'ArrowUp') {
        const mode = pico.getMode();
        pico.updateMode(mode === 'light' ? 'dark' : 'light');
        return;
      }

      if (isCommandKeyIntentfulPress && commandIntentKey === 'ArrowDown') {
        pico.updateTheme('default');
        return;
      }
    }

    document.addEventListener('keydown', onKeyDownEventHandler);
  }
}

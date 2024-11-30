const themeColors = [
  'default',
  'amber',
  'blue',
  'cyan',
  'fuchsia',
  'green',
  'grey',
  'indigo',
  'jade',
  'lime',
  'orange',
  'pink',
  'pumpkin',
  'purple',
  'red',
  'sand',
  'slate',
  'violet',
  'yellow',
  'zinc'
] as const;

export default themeColors;

export type ThemeColor = (typeof themeColors)[number];

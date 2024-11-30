Bun.build({
  entrypoints: ['lib/index.ts'],
  outdir: './out',
  target: 'browser',
  format: 'esm',
  splitting: false,
  sourcemap: 'linked',
  minify: true,
  external: []
});

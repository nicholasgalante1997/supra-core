type BunBuildConfig = Parameters<typeof Bun.build>[0];

export const base: BunBuildConfig = {
    entrypoints: ['lib/cli.ts'],
    banner: '#! /usr/bin/env bun',
    outdir: './out',
    target: 'bun',
    format: 'esm',
    splitting: false,
    sourcemap: 'linked',
    minify: true,
    external: ['react', 'react-dom']
};
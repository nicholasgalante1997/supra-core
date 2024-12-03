import React from 'react';
import * as ReactDOMServer from 'react-dom/server';

import path from 'path';

export async function loadComponent(pathlike: string, name: string = 'default', strategy?: 'eager' | 'lazy') {
    const resolved = path.resolve(pathlike.startsWith('/') ? pathlike : path.join(process.cwd(), pathlike));
    const loadedModule = await import(resolved);
    return loadedModule[name];
}

export async function pipeComponent(component: React.ReactElement) {
    const stream = await ReactDOMServer.renderToReadableStream(component);
    stream.
}
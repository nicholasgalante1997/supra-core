import React from "react";
import * as ReactDOMServer from "react-dom/server";

import path from "path";

export async function loadComponent(
  pathlike: string,
  name: string = "default"
) {
  const resolved = path.resolve(
    pathlike.startsWith("/") ? pathlike : path.join(process.cwd(), pathlike)
  );
  const loadedModule = await import(resolved);
  return loadedModule[name];
}

export async function pipeComponentToStdout(component: React.ReactElement, options: ReactDOMServer.RenderToReadableStreamOptions = {}) {
  const stream = await ReactDOMServer.renderToReadableStream(component, options);
  const reader = stream.getReader();
  const decoder = new TextDecoder();
  let result: any = { value: undefined, done: false };
  while (!result.done) {
    result = await reader.read()
    process.stdout.write(decoder.decode(result.value));
  }
}

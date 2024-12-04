# Debugrs

## Inspiration

A simple logger for Rust based on the Node.js [`debug`](https://npmjs.com/package/debug) lightweight logging library.

## Usage

```rust
use debugrs::RsDebugger;
 
let mut debugger = RsDebugger::new(String::from("app"));
debugger.write(String::from("App booting!... :rocket:")); // > [app] App booting!... 🚀 +0ms
 
let mut child = debugger.extend(String::from("child-process"));
child.write(String::from("Child process booting!... :rocket:")); // > [app:child-process] Child process booting!... 🚀 +0ms
```

## Feature Support

- Env based label filtering
- Emoji support (thanks to the `emojis` crate)
- Timed log events

## Conventions

If you're using this in one or more of your libraries, you should use the name of your library so that developers may toggle debugging as desired without guessing names. If you have more than one debuggers you should prefix them with your library name and use ":" to separate features. For example "bodyParser" from Connect would then be "connect:bodyParser". If you append a "*" to the end of your name, it will always be enabled regardless of the setting of the DEBUG environment variable. You can then use it for normal output as well as debug output.

## Wildcards

The * character may be used as a wildcard. Suppose for example your library has debuggers named "connect:bodyParser", "connect:compress", "connect:session", instead of listing all three with DEBUG=connect:bodyParser,connect:compress,connect:session, you may simply do DEBUG=connect:*, or to run everything using this module simply use DEBUG=*.

Environment Variables

To enable logging for all loggers or a specific subset of loggers, you can use the `DEBUG` environment variable.

```sh
export DEBUG=*
```

or if you are using a .env file

```txt
DEBUG=*
```

## License

MIT License

Copyright (c) [year] [fullname]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Contributing

Please fork this repo and create a pull request. For more details, see [`Contributing.md`](./Contributing.md).

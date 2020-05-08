# Hyo Client

## Building

### Prerequisites

In order to build the project you need to have a few things installed:

- [Rust](https://www.rust-lang.org/) and specifically Cargo
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
- [Node.js](https://nodejs.org/en/) and [npm](https://www.npmjs.com/)

But once you have them installed running it is as simple as:

```shell
npm run watch
```

This builds the site and starts a webserver. The site is rebuilt when changes are detected.
Currently recompiling the WebAssembly causes some problems. If changes to the Rust code don't seem to be taking effect just restart the watch task.

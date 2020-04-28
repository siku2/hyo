# Hyo

Just playing around with [Yew](https://yew.rs/) for a bit.
This project is currently in a very early phase. So much so that the goal of it hasn't even been established yet.

## Usage

### Build

Use [wasm-pack](https://rustwasm.github.io/wasm-pack/) to build the application:

```shell
wasm-pack build --target web
```

### Hosting

This is currently really basic.
You can find the files in the [site](site) directory but because it needs to access the output of wasm-pack you need to host the entire project directory.
For instance, I'm using the VS Code extension [Live Server](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer).

You can run the following command in the project root directory to start a simple HTTP server (assuming you have Python installed, of course).

```shell
python -m http.server
```

After running this command you should be able to access the site using the URL: <http://localhost:8000/site/>

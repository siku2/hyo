_ensure-sass-installed:
    #!/usr/bin/env sh
    if ! command -v sass >/dev/null 2>&1; then
        echo "ERROR: 'sass' not found.";
        echo "";
        echo "Sass doesn't appear to be installed.";
        echo "Please install Dart Sass (https://sass-lang.com/dart-sass) and make sure 'sass' is available in the path";
        echo "";
        exit 1;
    fi

install-dependencies:
    cargo install \
        simple-http-server \
        wasm-pack \
        watchexec
    
    just _ensure-sass-installed

server-run:
    just hyo-server/run
server-watch:
    just hyo-server/watch

client-serve:
    just hyo-client/serve
client-watch:
    just hyo-client/watch

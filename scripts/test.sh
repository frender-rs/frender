set -eux

cargo test -p frender-ssr-html
cargo test -p frender-ssr-html --features either

cargo test -p frender-ssr
cargo test -p frender-ssr --features either

cargo test -p frender-dom-tokens
cargo test -p frender-dom-tokens --features either
cargo test -p frender-dom-tokens --features web

cargo test -p frender-style
cargo test -p frender-style --features either
cargo test -p frender-style --features web

. ./scripts/test/frender-dom.sh
. ./scripts/test/frender-form-control.sh
. ./scripts/test/frender-html.sh

cargo test -p frender-render-with --features csr
cargo +nightly test -p frender-render-with --features csr,nightly

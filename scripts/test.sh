cargo test -p frender-ssr-html
cargo test -p frender-ssr-html --features either

cargo test -p frender-ssr
cargo test -p frender-ssr --features either

cargo test -p frender-render-with
cargo +nightly test -p frender-render-with --features nightly

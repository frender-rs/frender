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

group_features_impl () {
  local prefix=$1

  if [ -n "$2" ]; then
    local cur=$2
    shift 2
    group_features_impl "$prefix" "$@"
    group_features_impl "$prefix$cur," "$@"
  else
    echo $prefix
  fi
}

group_features () {
  group_features_impl "" "$@"
}

. ./scripts/test/frender-html.sh

cargo test -p frender-render-with
cargo +nightly test -p frender-render-with --features nightly

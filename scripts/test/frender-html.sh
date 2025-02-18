set -eu

while IFS= read -r line; do
  # either requires csr
  if [[ "$line" == *either,* ]] && [[ "$line" != *csr,* ]]; then
    echo "[SKIP ] $line"
    continue
  fi

  echo "[CHECK] $line"

  RUSTFLAGS=-Awarnings cargo check -p frender-html --features "$line"
done < <(group_features macros_not_expanded components csr ssr web either ElementProxyAttrs)

FEATURES=components,csr,ssr,web,either,ElementProxyAttrs
echo "[TEST ] $FEATURES"
cargo test -p frender-html --features "$FEATURES"
echo "[TEST ] --all-features"
cargo test -p frender-html --all-features

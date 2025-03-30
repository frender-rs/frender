set -eu

. ./scripts/features.sh

while IFS= read -r line; do
  echo "[CHECK] $line"
  cargo check -p frender-dom-tokens --all-targets --no-default-features --features "$line"
done < <(group_features csr ssr either web experimental)

cargo test -p frender-dom-tokens --all-features

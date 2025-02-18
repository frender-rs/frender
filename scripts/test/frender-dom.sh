set -eu

. ./scripts/features.sh

while IFS= read -r line; do
  echo "[CHECK] $line"

  RUSTFLAGS=-Awarnings cargo check -p frender-dom --features "$line"
done < <(group_features ssr csr web either)

echo "[TEST ] --all-features"
cargo test -p frender-dom --all-features

set -eu

. ./scripts/features.sh

while IFS= read -r line; do
  echo "[CHECK] $line"

  rust_flags=""
  if [[ "$line" == *csr,* ]]; then
    rust_flags="-Awarnings"
  fi
  RUSTFLAGS="$rust_flags" cargo check -p frender-synced-collection --features "$line"
done < <(group_features csr ssr ToElement)

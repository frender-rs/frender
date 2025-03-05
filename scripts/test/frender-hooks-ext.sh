set -eu

. ./scripts/features.sh

while IFS= read -r line; do
  echo "[CHECK] $line"

  cargo check -p frender-hooks-ext --features "$line"
done < <(group_features csr ssr ToElement either)

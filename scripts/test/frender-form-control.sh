set -eu

. ./scripts/features.sh

while IFS= read -r line; do
  echo "[CHECK] $line"

  RUSTFLAGS=-Awarnings cargo check -p frender-form-control --features "$line"
done < <(group_features csr ssr web chrono either)

while IFS= read -r line; do
  # wasm32-unknown-unknown only checks with web or chrono
  if [[ "$line" != *web,* ]] && [[ "$line" != *chrono,* ]]; then
    echo "[SKIP ] $line"
    continue
  fi

  echo "[CHECK] $line"

  # RUSTFLAGS=-Awarnings
  cargo check -p frender-form-control --features "$line" --target wasm32-unknown-unknown
done < <(group_features csr ssr web chrono either)

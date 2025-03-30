set -eu

. ./scripts/features.sh

while IFS= read -r line; do
  echo "[CHECK] $line"

  rust_flags=""
  if [[ "$line" == *csr,* ]]; then
    rust_flags="-Awarnings"
  fi
  rust_flags="-Awarnings"

  RUSTFLAGS="$rust_flags" cargo check -p frender --no-default-features --features "$line"
done < <(group_features spawn RenderWith csr ssr web html-components either hooks hooks_ext HookElement ToElement context SyncedCollection KeyedElements Memo)

```sh
cd examples/counter

wasm-pack build --dev -t web
# or
cargo watch -i pkg -s "wasm-pack build --dev -t web"

yarn parcel index.html
```

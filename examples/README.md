# Test examples

```sh
cd examples/counter
trunk serve
```

Then navigate to [http://127.0.0.1:8080] to checkout the example.

# Install `trunk`

```sh
cargo install trunk
```

`trunk` will download necessary prebuilt libraries when used.
If they are not available for your cpu or system,
you can manually install the libraries, for example:

```sh
cargo install wasm-bindgen-cli --version 0.2.78
```

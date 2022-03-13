# Quick Start

1.  Create a new cargo project

    ```sh
    cargo new my-frender-app
    cd my-frender-app
    ```

2.  Add `frender` to dependencies in `Cargo.toml`.

    ```toml
    [dependencies]
    frender = "= 1.0.0-alpha.8"
    ```

3.  Create `index.html` in the project root directory.

    ```html
    <!DOCTYPE html>
    <html>
      <head>
        <meta charset="UTF-8" />
        <title>My frender App</title>
        <script src="https://unpkg.com/react@17/umd/react.development.js"></script>
        <script src="https://unpkg.com/react-dom@17/umd/react-dom.development.js"></script>
        <link data-trunk rel="rust" href="Cargo.toml" />
      </head>
      <body>
        <div id="frender-root"></div>
      </body>
    </html>
    ```

4.  Modify `src/main.rs`

    ```rust
    use frender::prelude::*;

    #[component(main(mount_element_id = "frender-root"))]
    fn Main() {
        rsx!(
            <div>
                "Hello, frender!"
            </div>
        )
    }
    ```

5.  Run with `trunk`

    Install [trunk](https://trunkrs.dev/#install) and then execute:

    ```sh
    trunk serve
    ```

    Then you can navigate to `http://localhost:8080` to see your frender app.

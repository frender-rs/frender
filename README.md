# frender

[![Crates.io](https://img.shields.io/crates/v/frender?style=for-the-badge)](https://crates.io/crates/frender)
[![docs.rs](https://img.shields.io/docsrs/frender/latest?style=for-the-badge)](https://docs.rs/frender)
[![GitHub license](https://img.shields.io/github/license/frender-rs/frender?style=for-the-badge)](https://github.com/frender-rs/frender/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/frender-rs/frender?style=for-the-badge)](https://github.com/frender-rs/frender/stargazers)

<div style="text-align:center;margin:16px">

![frender logo](./logo.svg)

Functional Rendering: `React` in `Rust`

</div>

_**f**render_ is still in alpha and it's api might change.
For now it is recommended to specify the exact version in `Cargo.toml`.
Before updating, please see the full [changelog](https://github.com/frender-rs/frender/blob/alpha/CHANGELOG.md) in case there are breaking changes.

Development is at [alpha](https://github.com/frender-rs/frender/tree/alpha#readme) branch.

There are some example apps in
[`examples`](https://github.com/frender-rs/frender/tree/alpha/examples)
folder. You can preview them at [this site](https://frender-rs.github.io/frender/).

## Contributing

`frender` is open sourced at [GitHub](https://github.com/frender-rs/frender).
Pull requests and issues are welcomed.

You can also [sponsor me](https://ko-fi.com/equalma) and I would be very grateful :heart:

[![Buy Me a Coffee at ko-fi.com](https://cdn.ko-fi.com/cdn/kofi2.png?v=3)](https://ko-fi.com/N4N26J11L)

## Quick Start

1.  Create a new cargo project

    ```sh
    cargo new my-frender-app
    cd my-frender-app
    ```

2.  Add `frender` to dependencies in `Cargo.toml`.

    ```toml
    [dependencies]
    frender = "= 1.0.0-alpha.6"
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

## `rsx` syntax

### `rsx` element

```rust
use frender::prelude::*;

rsx! (
  <MyComp id="my-component">
    // Child node can be any literal strings or numbers
    "some string"
    1
    // Child node can be any rust expressions wrapped in braces
    { 1 + 6 }
    { value }
    // Child node can be an element
    <MyChild key="k" prop={any_expr} />

    // Prop without value means `true`, just like React
    <MyDialog show />

    // Fragment
    <>1 2 3</>
    // Fragment with key
    <# key="key">1 2 3</#>

    // you can also use `</_>` to enclose any element
    <path::to::Component></_>
    // the above is equivalent to:
    <path::to::Component></path::to::Component>
  </MyComp>
)
```

Any component name starting with lower case letter `[a-z]`
will be interpreted as an **intrinsic component**.
For example, `rsx!( <div id="my-div" /> )` will be resolved to:

```rust
use frender::prelude::*;
use self::intrinsic_components::div::prelude::*;

rsx! (
  <self::intrinsic_components::div::prelude::Component id="my-div" />
)
```

### `rsx` prop

In order to make rsx less verbose, frender provides
`IntoPropValue` trait. The `value` in
`<MyComponent prop={value} />` will be mapped to
`IntoPropValue::into_prop_value(value)`.

With this, assuming the prop accepts `Option<i32>`,
you can simplify `prop={Some(1)}` to `prop={1}`,
because `T` implements `IntoPropValue<Option<T>>`.

If you want to pass the value as is, you can
use `:=` to set prop. `prop:={value}`

## Write a component

### Component with no props

```rust
use frender::prelude::*;

#[component]
fn MyComponent() {
  //            ^
  //            the return type defaults to react::Element
  rsx!( <div /> )
}

// Or you can specify the return type explicitly
#[component]
fn MyAnotherComponent() -> Option<react::Element> {
  if check_something() {
    Some(rsx!( <MyComponent /> ))
  } else {
    None
  }
}
```

### Component with props

First, define `MyProps`

```rust
use frender::prelude::*;

def_props! {
  pub struct MyProps {
    // Required prop
    name: String,
    // Optional prop which defaults to `Default::default()`
    age: Option<u8>,
    // If the prop type is not specified,
    // then frender will infer the type by prop name.
    // For example, `class_name` default has type `Option<String>`
    class_name?,
    // Prop can also have type generics.
    // For example, the following is
    // the default definition for prop `children`
    children<TNode: react::Node>(value: Option<TNode>) -> Option<react::Children> {
      value.and_then(react::Node::into_children)
    },
  }
}
```

Then write the component with the above props:

```rust
use frender::prelude::*;
#[component]
pub fn MyComponent(props: &MyProps) {
  rsx!(<div>{&props.children}</div>)
}
```

Due to the generics, in some very rare cases, you may meet errors like
`type annotations needed` `cannot infer type for type parameter`.
You can solve it by specifying the type
with the turbofish syntax `::<>`.
For example:

```rust
rsx! (
  // ERROR: type annotations needed
  <a children={None} />
)
rsx! (
  // it works!
  <a children={None::<()>} />
)
```

## Hooks

React hooks are also available in `frender`.

You checkout the [examples](https://github.com/frender-rs/frender/blob/alpha/examples/counter/src/my_counter.rs) for the usage.

## Future Development Plans

- [ ] Documentation
- [ ] Intrinsic svg components
- [ ] Export `frender` components to js
- [ ] Server Side Rendering
- [ ] Type checking for `CssProperties`
- [ ] Css-in-rust (For example, integrate with [`emotion/react`](https://emotion.sh/docs/@emotion/react))

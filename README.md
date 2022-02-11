# frender

[![Crates.io](https://img.shields.io/crates/v/frender?style=for-the-badge)](https://crates.io/crates/frender)
[![docs.rs](https://img.shields.io/docsrs/frender/latest?style=for-the-badge)](https://docs.rs/frender)
[![GitHub license](https://img.shields.io/github/license/frender-rs/frender?style=for-the-badge)](https://github.com/frender-rs/frender/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/frender-rs/frender?style=for-the-badge)](https://github.com/frender-rs/frender/stargazers)

<div style="text-align:center;margin:16px">

![frender logo](./frender/logo.svg)

Functional Rendering: `React` in `Rust`

</div>

_Working in progress_

Development is at [alpha](https://github.com/frender-rs/frender/tree/alpha#readme) branch.

## rsx syntax

```rust
use frender::prelude::*;

rsx! (
  <MyComp id="my-component">
    // any literal
    "some string"
    1
    // any expression wrapped in braces
    { 1 + 6 }
    { value }
    // Child element
    <MyChild key="k" prop={any_expr} />
    // Fragment
    <>1 2 3</>
    // Fragment with key
    <# key="key">1 2 3</>
    // you can also use `</_>` to enclose any element
    <path::to::Component></_>
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
  <self::intrinsic_components::div::Component id="my-div" />
)
```

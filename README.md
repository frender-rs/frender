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
  <self::intrinsic_components::div::Component id="my-div" />
)
```

### `rsx` prop

In order to make rsx less verbose, frender provides
`IntoPropValue` trait. The `value` in
`<MyComponent prop={value} />` will be mapped to
`IntoPropValue::into_prop_value(value)`.

With this, assuming the prop is `Option<i32>`, you can
simplify `prop={Some(1)}` to `prop={1}`.

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
    // For example, `children` default has type `Option<String>`
    class_name?,
    // Prop can also have type generics.
    // For example, the following is
    // the default definition for prop `children`
    children<TNode: react::Node>(value: TNode) -> Option<react::Children> {
      value.into_react_children_js()
    },
  }
}
```

Then write the component with the above props:

```rust
use frender::prelude::*;
#[component]
pub fn MyComponent(props: MyProps) {
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

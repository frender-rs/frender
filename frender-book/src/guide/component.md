# Write a component

With the `#[component]` macro,
writing a component in frender is
as simple as writing a rust `fn`.
The macro will turn the `fn` item to a `struct`,
and implement
[`UseRenderStatic`](https://docs.rs/frender/latest/frender/prelude/trait.UseRenderStatic.html)
and
[`ComponentStatic`](https://docs.rs/frender/latest/frender/prelude/trait.ComponentStatic.html)
for it.
The `fn` body will be the body of `UseRenderStatic::use_render` method.
The return type of the `fn` will be `UseRenderStatic::RenderOutput`.
The optional first arg of the `fn` will be the props type of this component,
which is `UseRenderStatic::RenderArg` and [`ComponentStatic::Props`].
If there is no args in the `fn`, the props type will be
[`react::NoProps`](https://docs.rs/react-rs/latest/react/struct.NoProps.html).

The return type can be omitted.
If omitted, the default return type is
[`react::Element`](https://docs.rs/react-rs/latest/react/struct.Element.html).

## Component with no props

```rust,no_run
use frender::prelude::*;

#[component]
fn MyComponent() {
  //            ^
  //            the return type defaults to react::Element
  rsx!( <div /> )
}

# fn check_something() -> bool { true }

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

## Component with props

First, define props with `def_props` macro.

```rust,no_run
use frender::prelude::*;

def_props! {
  pub struct MyProps {
    // Required prop
    name: String,

    // Optional prop which defaults to `Default::default()`
    // The following property `age` is optional, and defaults to `None`
    age?: Option<u8>,

    // The following property `tags` is optional, and defaults to `Vec::default()`
    tags?: Vec<String>,

    // If the prop type is not specified,
    // then frender will infer the type by prop name.
    // For example, `class_name` default has type `Option<String>`
    // The following property `class_name` is optional, has type Option<String>
    class_name?,

    // The following property `id` is required, has type Option<String>
    id,

    // Prop can also have type generics.
    // For example, the following is
    // the default definition for prop `children`,
    // which means it accepts any `Option<TNode>` where TNode implements react::Node,
    // and then map the value into `Option<react::Children>` and store it into MyProps.
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
    rsx!(
        <div>{&props.children}</div>
    )
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

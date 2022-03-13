# `rsx` syntax

Frender provides the `rsx` macro to create elements
with syntax similar to jsx.

## Create elements with props

`rsx` prop key should be an valid ident.
Any literal string or expression wrapped in braces
can be used as `rsx` prop value.

```rust,no_run
rsx!(
    <MyComponent
        prop1="my-literal-string"
        prop2={my_expr}
        prop3
    />
)
```

The above code requires `MyComponent` to implements
[`StaticComponent`](https://docs.rs/react-rs/latest/react/trait.ComponentStatic.html),
and will be transformed into something like
the following code, which makes props strongly typed.

```rust,no_run
MyComponent::create_element(
    MyComponent::Props::init_builder()
        .prop1("my-literal-string".into_prop_value())
        .prop2(my_expr.into_prop_value())
        .prop3(true.into_prop_value())
        .build()
)
```

### Prop value auto conversion

Notice the `my_expr.into_prop_value()` in the above example.
[`IntoPropValue::into_prop_value`](https://docs.rs/react-rs/latest/react/trait.IntoPropValue.html)
trait is introduced for auto converting prop values,
so that rsx props can be less verbose.

For example, assuming the `index` prop of `MyComponent`
accepts `Option<i32>`,
you can create the element with any of the following codes.

```rust,no_run
rsx! ( <MyComponent index={Some(1)} /> )
rsx! ( <MyComponent index={1} /> )
```

`IntoPropValue<Option<T>>` is implemented for any type `T`.
Thus, `1` can be converted to `Some(1)`
automatically when used rsx prop value.

To disable the auto conversion,
you can use `:=` to set prop `prop:={value}`.
For example:

```rust,no_run
rsx!(
    <MyComponent prop1:="value" prop2="value" />
)
```

will be transformed into

```rust,no_run
MyComponent::create_element(
    MyComponent::Props::init_builder()
        .prop1("value")
        .prop2("value".into_prop_value())
        .build()
)
```

## Create elements with children

Any literals, rsx elements or rust expressions wrapped in braces
can be a valid child in rsx elements.

Elements with multiple children will accept a tuple of these values
into `children` prop.
For example:

```rust,no_run
rsx!(
    <MyComponent>
        "my-literal-string"
        1
        {my_expr}
        <div id="my-div" />
    </MyComponent>
)
```

is equivalent to the following code:

```rust,no_run
rsx!(
    <MyComponent
        children={
            (
                "my-literal-string",
                1,
                {my_expr},
                rsx!( <div id="my-div" /> ),
            )
        }
    />
)
```

Elements with exactly one child will accept a single value
into `children` prop, not wrapped in a tuple.
For example:

```rust,no_run
rsx!(
    <MyComponent>
        <div id="my-div" />
    </MyComponent>
)
```

will be transformed to the following code:

```rust,no_run
rsx!(
    <MyComponent
        children={rsx!( <div id="my-div" /> )}
    />
)
```

## Create intrinsic elements

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

Notice the `self::intrinsic_components`.
This means rsx searches for `intrinsic_components` module
in the current module instead of sub module of `frender`.
This would allow you to import a custom `intrinsic_components`
in a module level.

## Keyed element

`key` prop is preserved as a keyword, like React.js.
When `key` prop is specified, the element will be wrapped with
[`Keyed<TElement>`](https://docs.rs/react-rs/latest/react/struct.Keyed.html),
indicating the element is created with a key.
For example:

```rust,no_run
use react::{Element, Keyed};

let element: Element = rsx!( <div /> );
let keyed_element: Keyed<Element> = rsx!( <div key="my-div" /> );
```

## Fragment element

```rust,no_run
rsx!(
    <>
        "create"
        <i>"react"</i>
        "fragment"
        {"to wrap multiple children"}
    </>
)
```

The above rsx will be transformed into:

```rust,no_run
rsx!(
    <self::rsx_runtime::Fragment>
        "create"
        <i>"react"</i>
        "fragment"
        {"to wrap multiple children"}
    </self::rsx_runtime::Fragment>
)
```

To create fragment element with key,
you can use `#` as the component name.

```rust,no_run
rsx!(
    <# key="my-key">1 2 3</#>
)
```

Note that `Fragment` component outputs a custom element type
[`FragmentElement`](https://docs.rs/react-rs/latest/react/struct.FragmentElement.html).
It implements `Into<react::Element>`.

## Enclose any element with `</_>`

You can enclose any element with `</_>`,
which is very useful when the component type
has a long module path or complex generics.

```rust,no_run
rsx!(
    <>
        <path::to::Component></_>
        // the above is equivalent to:
        <path::to::Component></path::to::Component>

        <MyComponent<i32>>1</_>
        // the above is equivalent to:
        <MyComponent<i32>>1</MyComponent<i32>>
    </>
)
```

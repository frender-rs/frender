# How rsx macro works

As described in the [guide](../guide/rsx.html),
frender interprets `rsx!( <MyComponent prop={value} /> )` as the builder pattern:

```rust,ignore
MyComponent::create_element(
    MyComponent::Props::init_builder()
        .prop(value)
        .build()
)
```

But how does the builder pattern know whether
a prop is required or optional?
Here I will explain the implementation details.

## Completely optional props

If the properties of a props struct
are all optional, it would be easy to
implement the builder pattern.
We can use the struct itself as the builder type.

```rust
struct MyProps {
    // defaults to None
    optional_name: Option<String>,
    // Optional prop does not need to be Option<T>.
    // It just need to implement Default.
    // The following prop defaults to 0.
    optional_num: i32
}

impl frender::react::Props for MyProps {
    type InitialBuilder = Self;

    fn init_builder() -> Self {
        Self {
            optional_name: Default::default(),
            optional_num: Default::default(),
        }
    }
}

// builder methods
impl MyProps {
    fn optional_name(mut self, value: Option<String>) -> Self {
        self.optional_name = value;
        self
    }

    fn optional_num(mut self, value: i32) -> Self {
        self.optional_num = value;
        self
    }
}

impl frender::react::PropsBuilder<MyProps> for MyProps {
    fn build(self) -> MyProps {
        self
    }
}

fn main() {
    let props = MyProps::init_builder()
        .optional_name("frender".to_string())
        .build();

    assert_eq!(props.optional_name, "frender");
    assert_eq!(props.optional_num, 0);
}
```

## Props with required fields

```rust
struct PropertyAlreadySet<T>(T);
struct PropertyNotSet;

struct MyProps {
    // required property
    pub name: String,
    // optional property, defaults to None
    pub optional_message: Option<String>,
}

impl frender::react::Props for MyProps {
    type InitialBuilder = MyPropsBuilder;

    fn init_builder() -> MyPropsBuilder {
        MyPropsBuilder {
            // required field is initialized as `PropertyNotSet`
            name: PropertyNotSet,
            // optional field is initialized as default
            optional_message: Default::default(),
        }
    }
}

struct MyPropsBuilder<MyPropsBuilder__name> {
    name: MyPropsBuilder__name,
    optional_message: Option<String>,
}

impl<MyPropsBuilder__name> MyPropsBuilder<MyPropsBuilder__name> {
    pub fn name(self, value: String) -> MyPropsBuilder<PropertyAlreadySet<String>> {
        MyPropsBuilder {
            name: value,
            optional_message: self.optional_message,
        }
    }

    pub fn optional_message(mut self, value: Option<String>) -> Self {
        self.optional_message = value;
        self
    }
}

impl frender::react::PropsBuilder<MyProps> for MyPropsBuilder<PropertyAlreadySet<String>> {
    fn build(self) -> MyProps {
        MyProps {
            name: self.name.0,
            optional_message: self.optional_message,
        }
    }
}
```

## `def_props` macro

Using frender, you don't need to
implement the above yourself.
All of the details is encapsulated
in the `def_props` macro.

```rust
# extern crate frender;
use frender::prelude::*;

def_props! {
    struct MyProps {
        // required property
        pub name: String,
        // optional property, defaults to None
        pub optional_message?: Option<String>,
    }
}
```

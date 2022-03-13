# How Frender works

Frender is based on react's functional components (`React.FC`).
`React.FC` are just js functions with special restrictions (
such as must return `null | React.Element`,
must obey hooks rules).
What frender do is just to convert a rust function to a js function with [`wasm_bindgen::clousre::Closure`](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html).

However, the problem is that js doesn't know about
the ownership and lifetimes of data in rust.
Luckily, React components have life cycles.
In functional components,
the best way to do tasks (side effects)
in life cycles is using
[`React.useEffect`](https://reactjs.org/docs/hooks-effect.html)
like the following.

```js
function MyComponent() {
  React.useEffect(() => {
    // do side effects on mounted
    doSomething();

    return () => {
      // do side effects on unmounted
      doSomeCleanup();
    };
  }, []);
}
```

Frender bridges rust ownership within the component life cycle,
by persisting some data when
like the following:

```js
function MyComponent() {
  const refInitialized = React.useRef(false);
  if (!refInitialized.current) {
    // persist rust data with
    // std::mem::ManuallyDrop
    persistRustData();
    refInitialized.current = true;
  }
  React.useEffect(() => {
    return () => {
      // manually drop the persisted data
      // on unmounted
      dropPersistedRustData();
    };
  }, []);
}
```

With the above idea,
`frender::react::use_ref` can be implemented by guarding `React.useRef`
with the following pseudo code.

```rust,ignore
fn use_ref<T>(initial_value: T) {
    let js_ref_object = React.useRef();
    if js_ref_object.is_undefined() {
        // forget the initial value and use a number to represent it
        let key: usize = forget_and_get_key(initial_value);
        // numbers are safe and easy to be converted to a js number
        js_ref_object.current = key;
    }

    React.useEffect(|| {
        // return a cleanup function so that
        // the forgotten data will be dropped after component unmounted
        return || {
            // manually drop the forgotten data
            manually_drop_by_key(key);
        }
    }, []);

    let rust_ref_object = FrenderUseRefObject {
        // to get the current data,
        // just get the current key,
        // and then get the data
        current: || get_by_key(js_ref_object.current),
        set_current: |new_value| {
            let old_key = js_ref_object.current;
            manually_drop_by_key(old_key);
            let new_key = forget_and_get_key(new_value);
            js_ref_object.current = new_key;
        },
    };

    rust_ref_object
}
```

Frender guards other hooks and the component function
in the same way to keep the app memory safe.

Next, you can read about [_How `rsx` macro works_](rsx.html).

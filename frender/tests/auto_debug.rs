use wasm_bindgen_test::*;

// wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn auto_debug_props() {
    struct MyAnyType;

    let any_info = std::any::type_name::<MyAnyType>();
    let any_info = format!("{any_info} {{ ... }}");

    struct MyDebugType;
    static DEBUG_INFO: &str = "MyDebugType debug info";
    impl ::std::fmt::Debug for MyDebugType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{DEBUG_INFO}")
        }
    }

    static PROPS_INFO: &str = "MyPropsType debug info";
    struct MyPropsType;
    impl frender::DebugProps for MyPropsType {
        fn as_debug_props(&self) -> Option<wasm_bindgen::JsValue> {
            Some(wasm_bindgen::JsValue::from_str(PROPS_INFO))
        }
    }

    static PROPS_ALSO_DEBUG_INFO: &str = "MyPropsAlsoDebug debug info";
    struct MyPropsAlsoDebug;
    impl frender::DebugProps for MyPropsAlsoDebug {
        fn as_debug_props(&self) -> Option<wasm_bindgen::JsValue> {
            Some(wasm_bindgen::JsValue::from_str(PROPS_ALSO_DEBUG_INFO))
        }
    }
    impl ::core::fmt::Debug for MyPropsAlsoDebug {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "should call MyPropsAlsoDebug as DebugProps")
        }
    }

    assert_eq!(frender::auto_debug_props!(MyAnyType).unwrap(), any_info);
    assert_eq!(frender::auto_debug_props!(MyDebugType).unwrap(), DEBUG_INFO);
    assert_eq!(frender::auto_debug_props!(MyPropsType).unwrap(), PROPS_INFO);
    assert_eq!(
        frender::auto_debug_props!(MyPropsAlsoDebug).unwrap(),
        PROPS_ALSO_DEBUG_INFO
    );
}

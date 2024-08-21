mod csr {
    use web_sys::wasm_bindgen::{JsCast, UnwrapThrowExt};

    use crate::csr::CssStyleDeclaration;

    mod shims {
        use web_sys::wasm_bindgen::{self, prelude::*};

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_name = "CSSStyleDeclaration")]
            pub(super) type CssStyleDeclaration;
            #[wasm_bindgen(
                catch,
                method,
                structural,
                js_class = "CSSStyleDeclaration",
                js_name = "removeProperty"
            )]
            pub(super) fn remove_property(
                this: &CssStyleDeclaration,
                property: &str,
            ) -> Result<(), JsValue>;
        }
    }

    impl CssStyleDeclaration for web_sys::CssStyleDeclaration {
        fn remove_property_str(&mut self, property: &str) {
            self.unchecked_ref::<shims::CssStyleDeclaration>()
                .remove_property(property)
                .unwrap_throw()
        }

        fn set_property_str_with_value_str_and_priority(
            &mut self,
            property_name: &str,
            value: &str,
            priority: crate::csr::Priority,
        ) {
            web_sys::CssStyleDeclaration::set_property_with_priority(
                self,
                property_name,
                value,
                if priority.is_important() {
                    "important" // TODO: optimize
                } else {
                    ""
                },
            )
            .unwrap_throw()
        }
    }
}

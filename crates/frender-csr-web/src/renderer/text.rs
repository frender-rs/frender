use frender_common::try_behavior::TryBehavior;
use frender_html::{csr::web::Node, renderer::RenderTextFrom};

use super::Renderer;

mod js_shims {
    use js_sys::JsString;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type Text;

        #[wasm_bindgen(method, setter)]
        pub fn set_data(this: &Text, val: JsString);

        #[wasm_bindgen(js_name = Document)]
        pub type Document;

        #[wasm_bindgen(method, structural, js_class = "Document", js_name = createTextNode)]
        pub fn create_text_node(this: &Document, data: JsString) -> web_sys::Text;

        /// Calls `String(value)`
        #[wasm_bindgen(js_name = String)]
        pub fn js_string(value: JsValue) -> JsString;
    }
}

mod to_js_string {
    use wasm_bindgen::JsValue;

    pub(super) trait ToJsString {
        fn to_js_string(&self) -> js_sys::JsString;
    }

    impl ToJsString for char {
        fn to_js_string(&self) -> js_sys::JsString {
            From::from(*self)
        }
    }

    macro_rules! impl_for_each_of {
        (
            impl<__> $trait:ident for each_of! {$(
                $for_ty:ty
            ),* $(,)?}
            $t:tt
        ) => {$(
            impl $trait for $for_ty $t
        )*};
    }

    impl_for_each_of!(
        impl<__> ToJsString
            for each_of! {
                i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,
                f32, f64,
                bool,
            }
        {
            fn to_js_string(&self) -> js_sys::JsString {
                super::js_shims::js_string(JsValue::from(*self))
            }
        }
    );
}

mod to_text_node {
    use super::Renderer;

    pub(super) trait ToTextNode {
        fn to_text_node<TB: crate::try_behavior::TryBehavior>(
            &self,
            renderer: &mut Renderer<TB>,
        ) -> web_sys::Text;

        fn update_text_node<TB: crate::try_behavior::TryBehavior>(
            &self,
            renderer: &mut Renderer<TB>,
            text: &web_sys::Text,
        );
    }

    impl<V: ?Sized + super::to_js_string::ToJsString> ToTextNode for V {
        fn to_text_node<TB: crate::try_behavior::TryBehavior>(
            &self,
            renderer: &mut Renderer<TB>,
        ) -> web_sys::Text {
            use wasm_bindgen::JsCast;
            super::js_shims::Document::create_text_node(
                renderer.document.unchecked_ref(),
                self.to_js_string(),
            )
        }

        fn update_text_node<TB: crate::try_behavior::TryBehavior>(
            &self,
            _: &mut Renderer<TB>,
            text: &web_sys::Text,
        ) {
            use wasm_bindgen::JsCast;
            super::js_shims::Text::set_data(text.unchecked_ref(), self.to_js_string())
        }
    }

    impl ToTextNode for str {
        fn to_text_node<TB: crate::try_behavior::TryBehavior>(
            &self,
            renderer: &mut Renderer<TB>,
        ) -> web_sys::Text {
            renderer.document.create_text_node(self)
        }

        fn update_text_node<TB: crate::try_behavior::TryBehavior>(
            &self,
            _: &mut Renderer<TB>,
            text: &web_sys::Text,
        ) {
            text.set_data(self)
        }
    }
}

impl<V: ?Sized + to_text_node::ToTextNode, TB: TryBehavior> RenderTextFrom<Node<web_sys::Text>, V>
    for Renderer<'_, TB>
{
    fn render_text_from(&mut self, v: &V) -> Node<web_sys::Text> {
        Node(v.to_text_node(self))
    }

    fn update_text_from(&mut self, text: &mut Node<web_sys::Text>, v: &V) {
        v.update_text_node(self, &text.0)
    }
}

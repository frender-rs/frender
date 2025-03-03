use frender_html::dom::csr::{render::RenderTextFrom, web::Node};

use super::Renderer;

mod js_shims {
    use js_sys::JsString;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type Text;

        #[wasm_bindgen(method, setter)]
        pub fn set_data(this: &Text, val: JsString);

        #[wasm_bindgen(method, setter = data)]
        pub fn set_data_ref(this: &Text, val: &JsString);

        #[wasm_bindgen(js_name = Document)]
        pub type Document;

        #[wasm_bindgen(method, structural, js_class = "Document", js_name = createTextNode)]
        pub fn create_text_node(this: &Document, data: JsString) -> web_sys::Text;

        #[wasm_bindgen(method, structural, js_class = "Document", js_name = createTextNode)]
        pub fn create_text_node_ref(this: &Document, data: &JsString) -> web_sys::Text;

        /// Calls `String(value)`
        #[wasm_bindgen(js_name = String)]
        pub fn js_string(value: JsValue) -> JsString;
    }
}

mod into_js_string {
    use wasm_bindgen::JsValue;

    pub(super) trait IntoJsString {
        fn into_js_string(self) -> js_sys::JsString;
    }

    impl IntoJsString for char {
        fn into_js_string(self) -> js_sys::JsString {
            From::from(self)
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
        impl<__> IntoJsString
            for each_of! {
                i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,
                f32, f64,
            }
        {
            fn into_js_string(self) -> js_sys::JsString {
                super::js_shims::js_string(JsValue::from(self))
            }
        }
    );
}

mod into_text_node {
    use std::{borrow::Cow, rc::Rc, sync::Arc};

    use frender_reactive_value::{static_or_temp_ref::StaticOrTempRef, temp_ref::TempRef};
    use wasm_bindgen::JsCast as _;

    use frender_common::impl_many;
    use frender_html::dom::string_element::StringElement;

    use super::Renderer;

    pub(super) trait IntoTextNode {
        fn into_text_node(self, renderer: &mut Renderer) -> web_sys::Text;

        fn update_text_node(self, renderer: &mut Renderer, text: &web_sys::Text);
    }

    impl<V: super::into_js_string::IntoJsString> IntoTextNode for V {
        fn into_text_node(self, renderer: &mut Renderer) -> web_sys::Text {
            super::js_shims::Document::create_text_node(
                renderer.document.unchecked_ref(),
                self.into_js_string(),
            )
        }

        fn update_text_node(self, _: &mut Renderer, text: &web_sys::Text) {
            super::js_shims::Text::set_data(text.unchecked_ref(), self.into_js_string())
        }
    }

    impl_many!(
        impl<__> IntoTextNode
            for each_of![
                //
                &'static str,
                String,
                Cow<'static, str>,
                TempRef<'_, str>,
                StaticOrTempRef<'_, str>,
                Rc<str>,
                TempRef<'_, Rc<str>>,
                Arc<str>,
                TempRef<'_, Arc<str>>,
            ]
        {
            fn into_text_node(self, renderer: &mut Renderer) -> web_sys::Text {
                renderer.document.create_text_node(self.as_ref())
            }

            fn update_text_node(self, _: &mut Renderer, text: &web_sys::Text) {
                text.set_data(self.as_ref())
            }
        }
    );

    impl IntoTextNode for StringElement {
        fn into_text_node(self, renderer: &mut Renderer) -> web_sys::Text {
            match self.into_js_string() {
                Ok(this) => super::js_shims::Document::create_text_node(
                    renderer.document.unchecked_ref(),
                    this,
                ),
                Err(this) => renderer.document.create_text_node(&this),
            }
        }

        fn update_text_node(self, _: &mut Renderer, text: &web_sys::Text) {
            match self.into_js_string() {
                Ok(this) => super::js_shims::Text::set_data(text.unchecked_ref(), this),
                Err(this) => text.set_data(&this),
            }
        }
    }

    impl IntoTextNode for TempRef<'_, StringElement> {
        fn into_text_node(self, renderer: &mut Renderer) -> web_sys::Text {
            match self.0.as_js_string() {
                Ok(this) => super::js_shims::Document::create_text_node_ref(
                    renderer.document.unchecked_ref(),
                    this,
                ),
                Err(this) => renderer.document.create_text_node(&this),
            }
        }

        fn update_text_node(self, _: &mut Renderer, text: &web_sys::Text) {
            match self.0.as_js_string() {
                Ok(this) => super::js_shims::Text::set_data_ref(text.unchecked_ref(), this),
                Err(this) => text.set_data(&this),
            }
        }
    }
}

impl<V: into_text_node::IntoTextNode> RenderTextFrom<V> for Renderer {
    type Text = Node<web_sys::Text>;

    fn render_text_from(render_context: &mut Self::RenderContext<'_>, v: V) -> Self::Text {
        let text = v.into_text_node(render_context.renderer);
        <Self as frender_html::dom::csr::web::Renderer>::mount_node(render_context, &text);
        Node(text)
    }

    fn update_text_from(&mut self, text: &mut Self::Text, v: V) {
        v.update_text_node(self, &text.0)
    }
}

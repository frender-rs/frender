use std::borrow::Cow;

use frender_html::{renderer::RenderTextFrom, RenderHtml};
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::try_behavior::{TryBehavior, TryWithTryBehavior};

#[derive(Debug, Clone)]
enum NextNodePosition<'a> {
    FirstChildOf(Cow<'a, web_sys::Element>),
    InsertAfter(Cow<'a, web_sys::Node>),
}

pub struct Renderer<'a, TB: TryBehavior> {
    document: &'a web_sys::Document,
    next_node_position: NextNodePosition<'a>,
    try_behavior: TB,
}

impl<'a, TB: TryBehavior> Renderer<'a, TB> {
    pub async fn render_element<E: frender_html::Element>(self, element: E) {
        let render = self.into_render_element();

        futures_lite::pin!(render);

        render.as_mut().update_with_element(element);

        render.await
    }
}

impl<'a> Renderer<'a, crate::try_behavior::UnwrapThrow> {
    pub fn new(document: &'a web_sys::Document, root_parent: web_sys::Element) -> Self {
        Self {
            document,
            next_node_position: NextNodePosition::FirstChildOf(Cow::Owned(root_parent)),
            try_behavior: crate::try_behavior::UnwrapThrow,
        }
    }
}

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

impl<TB: TryBehavior> RenderHtml for Renderer<'_, TB> {
    type EventListenerId = gloo_events::EventListener;
    type Event = web_sys::Event;
    type Text = Node<web_sys::Text>;

    type div = Node<web_sys::HtmlDivElement>;

    fn div(&mut self) -> Self::div {
        let element = self
            .document
            .create_element(<frender_html::element_types::div as frender_html::renderer::HasIntrinsicComponentTag>::INTRINSIC_COMPONENT_TAG)
            .unwrap_with_behavior(&mut self.try_behavior);
        Node(element.unchecked_into())
    }
}

pub struct Node<N>(N);

mod node {
    use std::borrow::Cow;

    use crate::try_behavior::{TryBehavior, TryWithTryBehavior};

    use super::{NextNodePosition, Renderer};

    impl<'a, TB: TryBehavior> Renderer<'a, TB> {
        fn readd_node(&mut self, node: &web_sys::Node, force_reposition: bool) {
            web_sys::console::log_3(&"readd_node".into(), node, &force_reposition.into());
            if force_reposition {
                match &self.next_node_position {
                    NextNodePosition::FirstChildOf(parent) => {
                        web_sys::console::log_2(&"FirstChildOf".into(), parent);

                        parent
                            .prepend_with_node_1(node)
                            .unwrap_with_behavior(&mut self.try_behavior)
                    }
                    NextNodePosition::InsertAfter(pre) => {
                        web_sys::console::log_2(&"InsertAfter".into(), pre);

                        pre.parent_node()
                            .unwrap()
                            .insert_before(node, pre.next_sibling().as_ref())
                            .unwrap_with_behavior(&mut self.try_behavior);
                    }
                }
            }

            self.next_node_position = NextNodePosition::InsertAfter(Cow::Owned(node.clone()));
        }

        fn remove_node(&mut self, node: &web_sys::Node) {
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            extern "C" {
                type Removable;

                #[wasm_bindgen(method, structural)]
                pub fn remove(this: &Removable);
            }

            node.unchecked_ref::<Removable>().remove()
        }
    }

    impl<N: AsRef<web_sys::Node>, TB: TryBehavior>
        frender_html::renderer::node_behaviors::Node<Renderer<'_, TB>> for super::Node<N>
    {
        fn cursor_is_at_self(&self, renderer: &Renderer<'_, TB>) -> bool {
            match &renderer.next_node_position {
                NextNodePosition::FirstChildOf(parent) => parent.first_child(),
                NextNodePosition::InsertAfter(previous) => previous.next_sibling(),
            }
            .map_or(false, |c| *self.0.as_ref() == c)
        }

        fn move_cursor_after_self(&mut self, renderer: &mut Renderer<'_, TB>) {
            let node = self.0.as_ref().clone();
            renderer.next_node_position = NextNodePosition::InsertAfter(Cow::Owned(node));
        }

        fn readd_self(&mut self, renderer: &mut Renderer<'_, TB>, force_reposition: bool) {
            renderer.readd_node(self.0.as_ref(), force_reposition)
        }

        fn remove_self(&mut self, renderer: &mut Renderer<'_, TB>) {
            renderer.remove_node(self.0.as_ref())
        }
    }

    impl<N: AsRef<web_sys::Element> + AsRef<web_sys::Node>, TB: TryBehavior>
        frender_html::renderer::node_behaviors::Element<Renderer<'_, TB>> for super::Node<N>
    {
        fn move_cursor_at_the_first_child_of_self(&mut self, renderer: &mut Renderer<'_, TB>) {
            let element: &web_sys::Element = self.0.as_ref();

            renderer.next_node_position =
                NextNodePosition::FirstChildOf(Cow::Owned(element.clone()));
        }

        fn set_attribute(&mut self, renderer: &mut Renderer<'_, TB>, name: &str, value: &str) {
            let element: &web_sys::Element = self.0.as_ref();

            element
                .set_attribute(name, value)
                .unwrap_with_behavior(&mut renderer.try_behavior)
        }

        fn remove_attribute(&mut self, renderer: &mut Renderer<'_, TB>, name: &str) {
            let element: &web_sys::Element = self.0.as_ref();
            element
                .remove_attribute(name)
                .unwrap_with_behavior(&mut renderer.try_behavior)
        }

        fn update_class_list(
            &mut self,
            renderer: &mut Renderer<'_, TB>,
            updater: impl frender_html::renderer::node_behaviors::UpdateDomTokenList,
        ) {
            let element: &web_sys::Element = self.0.as_ref();

            updater.update_dom_token_list(
                renderer,
                &mut super::dom_token_list::DomTokenList(element.class_list()),
            )
        }

        fn set_id(&mut self, renderer: &mut Renderer<'_, TB>, id: &str) {
            let element: &web_sys::Element = self.0.as_ref();

            element.set_id(id)
        }
    }

    impl<
            N: AsRef<web_sys::HtmlElement> + AsRef<web_sys::Element> + AsRef<web_sys::Node>,
            TB: TryBehavior,
        > frender_html::renderer::node_behaviors::HtmlElement<Renderer<'_, TB>> for super::Node<N>
    {
        fn set_access_key(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            let element: &web_sys::HtmlElement = self.0.as_ref();
            element.set_access_key(value)
        }

        fn set_content_editable(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            let element: &web_sys::HtmlElement = self.0.as_ref();
            element.set_content_editable(value)
        }
    }
}

mod dom_token_list {
    use crate::try_behavior::{TryBehavior, TryWithTryBehavior};

    use super::Renderer;

    pub(super) struct DomTokenList(pub(super) web_sys::DomTokenList);

    impl<TB: TryBehavior> frender_html::renderer::node_behaviors::DomTokenList<Renderer<'_, TB>>
        for DomTokenList
    {
        fn set_value(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            self.0.set_value(value)
        }

        fn add_1(&mut self, renderer: &mut Renderer<'_, TB>, token: &str) {
            self.0
                .add_1(token)
                .unwrap_with_behavior(&mut renderer.try_behavior)
        }

        fn remove_1(&mut self, renderer: &mut Renderer<'_, TB>, token: &str) {
            self.0
                .remove_1(token)
                .unwrap_with_behavior(&mut renderer.try_behavior)
        }

        fn replace(&mut self, renderer: &mut Renderer<'_, TB>, old_token: &str, new_token: &str) {
            _ = self
                .0
                .replace(old_token, new_token)
                .unwrap_with_behavior(&mut renderer.try_behavior)
        }
    }
}

use web_sys::{
    js_sys::JsString,
    wasm_bindgen::{self, prelude::*},
};

use crate::{
    csr::{
        render_from::{str::ValueForStr, RenderFrom},
        web::{self, Node},
    },
    string_element::StringElement,
};

use super::SetInnerHtmlFromStr;

fn set_inner_html_from_str(element: &Node<impl AsRef<web_sys::Element>>, value: &str) {
    AsRef::<web_sys::Element>::as_ref(&element.0).set_inner_html(value)
}

#[wasm_bindgen]
extern "C" {
    type SetInnerHtml;
    #[wasm_bindgen(structural, method, setter, js_name = "innerHTML")]
    fn set_inner_html_borrowed(this: &SetInnerHtml, value: &JsString);
    #[wasm_bindgen(structural, method, setter, js_name = "innerHTML")]
    fn set_inner_html_owned(this: &SetInnerHtml, value: JsString);
}

fn as_set_inner_html(element: &Node<impl AsRef<web_sys::Element>>) -> &SetInnerHtml {
    AsRef::<web_sys::Element>::as_ref(&element.0).unchecked_ref::<SetInnerHtml>()
}

struct RenderSetInnerHtml<'a, N: AsRef<web_sys::Element>>(&'a Node<N>);

impl<'a, N: AsRef<web_sys::Element>, V: AsRef<str>> RenderFrom<V> for RenderSetInnerHtml<'a, N> {
    fn render_from(self, value: V) {
        set_inner_html_from_str(self.0, value.as_ref())
    }
}

impl<'a, N: AsRef<web_sys::Element>> RenderFrom<StringElement> for RenderSetInnerHtml<'a, N> {
    fn render_from(self, value: StringElement) {
        match value.into_js_string() {
            Ok(value) => as_set_inner_html(self.0).set_inner_html_owned(value),
            Err(v) => self.render_from(v),
        }
    }
}

impl<'a, N: AsRef<web_sys::Element>> RenderFrom<&StringElement> for RenderSetInnerHtml<'a, N> {
    fn render_from(self, value: &StringElement) {
        match value.as_js_string() {
            Ok(value) => as_set_inner_html(self.0).set_inner_html_borrowed(value),
            Err(value) => self.render_from(value),
        }
    }
}

impl<N: AsRef<web_sys::Node> + AsRef<web_sys::Element>, Renderer: ?Sized + web::Renderer>
    SetInnerHtmlFromStr<Renderer> for Node<N>
{
    fn set_inner_html_from_str(&mut self, _: &mut Renderer, value: impl ValueForStr) {
        value.render_str_from_self(RenderSetInnerHtml(self))
    }
}

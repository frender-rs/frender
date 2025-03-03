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

use super::SetInnerTextFromStr;

fn set_inner_text_from_str(element: &Node<impl AsRef<web_sys::HtmlElement>>, value: &str) {
    element.0.as_ref().set_inner_text(value)
}

#[wasm_bindgen]
extern "C" {
    type SetInnerText;
    #[wasm_bindgen(structural, method, setter, js_name = "innerText")]
    fn set_inner_text_borrowed(this: &SetInnerText, value: &JsString);
    #[wasm_bindgen(structural, method, setter, js_name = "innerText")]
    fn set_inner_text_owned(this: &SetInnerText, value: JsString);
}

fn as_set_inner_text(element: &Node<impl AsRef<web_sys::HtmlElement>>) -> &SetInnerText {
    element.0.as_ref().unchecked_ref::<SetInnerText>()
}

struct RenderSetInnerText<'a, N: AsRef<web_sys::HtmlElement>>(&'a Node<N>);

impl<'a, N: AsRef<web_sys::HtmlElement>, V: AsRef<str>> RenderFrom<V>
    for RenderSetInnerText<'a, N>
{
    fn render_from(self, value: V) {
        set_inner_text_from_str(self.0, value.as_ref())
    }
}

impl<'a, N: AsRef<web_sys::HtmlElement>> RenderFrom<StringElement> for RenderSetInnerText<'a, N> {
    fn render_from(self, value: StringElement) {
        match value.into_js_string() {
            Ok(value) => as_set_inner_text(self.0).set_inner_text_owned(value),
            Err(v) => self.render_from(v),
        }
    }
}

impl<'a, N: AsRef<web_sys::HtmlElement>> RenderFrom<&StringElement> for RenderSetInnerText<'a, N> {
    fn render_from(self, value: &StringElement) {
        match value.as_js_string() {
            Ok(value) => as_set_inner_text(self.0).set_inner_text_borrowed(value),
            Err(value) => self.render_from(value),
        }
    }
}

impl<
        N: AsRef<web_sys::Node> + AsRef<web_sys::Element> + AsRef<web_sys::HtmlElement>,
        Renderer: ?Sized + web::Renderer,
    > SetInnerTextFromStr<Renderer> for Node<N>
{
    fn set_inner_text_from_str(&mut self, _: &mut Renderer, value: impl ValueForStr) {
        value.render_str_from_self(RenderSetInnerText(self))
    }
}

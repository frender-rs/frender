use frender::{react, Children};
use wasm_bindgen::JsValue;

pub struct AnchorProps {
    href: String,
    children: frender::AnyChildren,
}

pub fn a(props: AnchorProps) -> react::sys::Element {
    let p = js_sys::Object::new();

    js_sys::Reflect::set(&p, &"href".into(), &props.href.into()).unwrap();

    crate::create_element_html("a", p.as_ref(), &props.children.into_react_children())
}

pub struct DivProps {
    pub children: frender::AnyChildren,
}

pub fn div(props: DivProps) -> react::sys::Element {
    crate::create_element_html("div", &JsValue::NULL, &props.children.into_react_children())
}

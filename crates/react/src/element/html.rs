use crate::{Element, IntrinsicElement};

/// Corresponding to [`ReactHTMLElement`]
///
/// [`ReactHTMLElement`]: https://github.com/DefinitelyTyped/DefinitelyTyped/blob/54d540ab4deb2588c0eff39dadf370cbf0a2dee4/types/react/v16/index.d.ts#L176
pub struct HtmlElement<TElement, Value> {
    tag: String,
}

impl<TElement, Value> Element for HtmlElement<TElement, Value> {
    fn as_react_element(&self) -> react_sys::Element {}
}

impl<TElement, Value> HtmlElement<TElement, Value> {}

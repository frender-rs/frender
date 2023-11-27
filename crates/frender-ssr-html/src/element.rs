use crate::assert::{HtmlChildren, SpaceAndHtmlAttributesOrEmpty, TagName};

async_str_iter::Strings!(
    enum NormalElementState {}
    /// https://html.spec.whatwg.org/multipage/syntax.html#normal-elements
    pub struct NormalElement<T: TagName, Attrs: SpaceAndHtmlAttributesOrEmpty, Children: HtmlChildren>(
        lt!("<"),
        tag!(T),
        attrs!(Attrs),
        gt!(">"),
        children!(Children),
        lt_slash!("</"),
        tag_close!(T),
        gt_close!(">"),
    );
);

impl<T: TagName, Attrs: SpaceAndHtmlAttributesOrEmpty, Children: HtmlChildren>
    NormalElement<T, Attrs, Children>
{
    pub const fn new(tag: T, attrs: Attrs, children: Children) -> Self
    where
        T: Copy,
    {
        Self {
            _state: NormalElementState(),
            lt: (),
            tag,
            attrs,
            gt: (),
            children,
            lt_slash: (),
            tag_close: tag,
            gt_close: (),
        }
    }
}

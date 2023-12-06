use crate::assert::{HtmlChildren, ScriptContent, SpaceAndHtmlAttributesOrEmpty, TagName};

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

async_str_iter::Strings!(
    enum ScriptElementState {}
    /// https://html.spec.whatwg.org/multipage/syntax.html#normal-elements
    pub struct ScriptElement<Attrs: SpaceAndHtmlAttributesOrEmpty, Children: ScriptContent>(
        tag_start!("<script"),
        attrs!(Attrs),
        gt!(">"),
        children!(Children),
        tag_end!("</script>"),
    );
);

impl<Attrs: SpaceAndHtmlAttributesOrEmpty, Children: ScriptContent> ScriptElement<Attrs, Children> {
    pub const fn new(attrs: Attrs, children: Children) -> Self {
        Self {
            _state: ScriptElementState(),
            tag_start: (),
            attrs,
            gt: (),
            children,
            tag_end: (),
        }
    }
}

async_str_iter::Strings!(
    enum VoidElementState {}
    /// https://html.spec.whatwg.org/multipage/syntax.html#normal-elements
    pub struct VoidElement<T: TagName, Attrs: SpaceAndHtmlAttributesOrEmpty>(
        lt!("<"),
        tag!(T),
        attrs!(Attrs),
        gt!(">"),
    );
);

impl<T: TagName, Attrs: SpaceAndHtmlAttributesOrEmpty> VoidElement<T, Attrs> {
    pub const fn new(tag: T, attrs: Attrs) -> Self {
        Self {
            _state: VoidElementState(),
            lt: (),
            tag,
            attrs,
            gt: (),
        }
    }
}

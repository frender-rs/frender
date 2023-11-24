use frender_ssr::{AsyncStrIterator, EscapeSafe, IntoAsyncStrIterator};
pub use props::{ElementProps, IntoElementProps};

mod props;

pub trait HasIntrinsicComponentTag {
    const INTRINSIC_COMPONENT_TAG: &'static str;
}

pub trait HasIntrinsicElementType {
    type IntrinsicElementType: IntrinsicElementType;
}

pub trait IntrinsicElementType {}

pub trait SsrIntrinsicComponent {
    #[inline]
    fn wrap_children<Children>(
        children: Children,
    ) -> frender_ssr::element::html::HtmlElementChildren<Children> {
        frender_ssr::element::html::HtmlElementChildren::Children(children)
    }
}

pub trait SsrSupportChildren<Children> {
    type ChildrenSsrState: frender_ssr::SsrRenderState;

    fn children_into_ssr_state(children: Children) -> Self::ChildrenSsrState;
}

impl<C: SsrIntrinsicComponent, Children: frender_ssr::SsrElement> SsrSupportChildren<Children>
    for C
{
    type ChildrenSsrState = Children::SsrState;

    fn children_into_ssr_state(children: Children) -> Self::ChildrenSsrState {
        Children::into_ssr_state(children)
    }
}

pub struct IntrinsicElement<C, P: IntoElementProps>(pub C, pub P);

frender_ssr::Strings!(
    enum IntrinsicElementEnclosingIterState {}
    pub struct IntrinsicElementEnclosingIter<
        C: AsyncStrIterator,
        Attrs: AsyncStrIterator,
        Children: AsyncStrIterator,
    >(
        lt!("<"),
        tag!(C),
        attrs!(Attrs),
        gt!(">"),
        children!(Children),
        lt_close!("<"),
        tag_close!(C),
        gt_close!(">"),
    );
);

pub trait IntoAsyncAttributeIterator {
    type IntoAsyncAttributeIterator: AsyncStrIterator;
    fn into_async_attribute_iterator(self) -> Self::IntoAsyncAttributeIterator;
}

impl IntoAsyncAttributeIterator for () {
    type IntoAsyncAttributeIterator = frender_ssr::Empty;

    fn into_async_attribute_iterator(self) -> Self::IntoAsyncAttributeIterator {
        frender_ssr::Empty
    }
}

impl<A: IntoAsyncAttributeIterator, B: IntoAsyncAttributeIterator> IntoAsyncAttributeIterator
    for (A, B)
{
    type IntoAsyncAttributeIterator =
        frender_ssr::Chain<A::IntoAsyncAttributeIterator, B::IntoAsyncAttributeIterator>;

    fn into_async_attribute_iterator(self) -> Self::IntoAsyncAttributeIterator {
        frender_ssr::Chain::new(
            self.0.into_async_attribute_iterator(),
            self.1.into_async_attribute_iterator(),
        )
    }
}

frender_ssr::Strings!(
    enum IterAttrPairState {}
    pub struct IterAttrPair<N: AsyncStrIterator, V: AsyncStrIterator>(
        space!(" "),
        name!(frender_ssr::Encode::<frender_ssr::UnquotedAttribute, N>),
        eq_quote!("=\""),
        value!(frender_ssr::Encode::<frender_ssr::DoubleQuotedAttribute, V>),
        quote!("\""),
    );
);

pub struct AttrPair<N: IntoAsyncStrIterator, V: IntoAsyncStrIterator>(pub N, pub V);

impl<N: IntoAsyncStrIterator, V: IntoAsyncStrIterator> IntoAsyncStrIterator for AttrPair<N, V> {
    type IntoAsyncStrIterator = IterAttrPair<N::IntoAsyncStrIterator, V::IntoAsyncStrIterator>;

    fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
        IterAttrPair {
            _state: IterAttrPairState(),
            space: (),
            name: frender_ssr::Encode::new(
                frender_ssr::UnquotedAttribute,
                self.0.into_async_str_iterator(),
            ),
            eq_quote: (),
            value: frender_ssr::Encode::new(
                frender_ssr::DoubleQuotedAttribute,
                self.1.into_async_str_iterator(),
            ),
            quote: (),
        }
    }
}

impl<C, P: IntoElementProps> frender_ssr::IntoAsyncStrIterator for IntrinsicElement<C, P>
where
    C: HasIntrinsicComponentTag,
    P::Attrs: IntoAsyncAttributeIterator,
    P::Children: IntoAsyncStrIterator,
{
    type IntoAsyncStrIterator = IntrinsicElementEnclosingIter<
        Option<&'static str>,
        <P::Attrs as IntoAsyncAttributeIterator>::IntoAsyncAttributeIterator,
        <P::Children as IntoAsyncStrIterator>::IntoAsyncStrIterator,
    >;

    fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
        let ElementProps {
            children,
            attributes,
        } = P::into_element_props(self.1);
        IntrinsicElementEnclosingIter {
            _state: IntrinsicElementEnclosingIterState(),
            lt: (),
            tag: Some(C::INTRINSIC_COMPONENT_TAG),
            attrs: attributes.into_async_attribute_iterator(),
            gt: (),
            children: children.into_async_str_iterator(),
            lt_close: (),
            tag_close: Some(C::INTRINSIC_COMPONENT_TAG),
            gt_close: (),
        }
    }
}

pub struct IntrinsicChildrenAsElement<C, Children> {
    pub component_type: C,
    pub children: Children,
}

mod ssr {
    use std::borrow::Cow;

    use frender_common::write::attrs::IntoAsyncWritableAttrs;
    // use frender_html::dom::component::ElementProps;
    use frender_ssr::{Element, IntoAsyncStrIterator};

    use super::{
        ElementProps, HasIntrinsicComponentTag, IntoAsyncAttributeIterator, IntoElementProps,
        IntrinsicChildrenAsElement, IntrinsicElementEnclosingIter, SsrIntrinsicComponent,
        SsrSupportChildren,
    };

    impl<C: SsrSupportChildren<Children>, Children> Element
        for IntrinsicChildrenAsElement<C, Children>
    {
        type SsrState = C::ChildrenSsrState;

        fn into_ssr_state(self) -> Self::SsrState {
            C::children_into_ssr_state(self.children)
        }

        type IntoAsyncHtmlChunks = Option<&'static str>;

        fn into_async_html_chunks(self) -> Self::IntoAsyncHtmlChunks {
            todo!()
        }
    }

    impl<C, P: IntoElementProps> Element for super::IntrinsicElement<C, P>
    where
        // C: super::SsrSupportChildren<P::Children>,
        C: SsrIntrinsicComponent + HasIntrinsicComponentTag,
        P::Attrs: IntoAsyncWritableAttrs,
        P::Children: Element,
        P::Attrs: IntoAsyncAttributeIterator,
        P::Children: IntoAsyncStrIterator,
    {
        type SsrState = ::frender_ssr::element::html::HtmlElementRenderState<
            'static,
            <C as super::SsrSupportChildren<P::Children>>::ChildrenSsrState,
            <P::Attrs as IntoAsyncWritableAttrs>::AsyncWritableAttrs,
        >;

        fn into_ssr_state(self) -> Self::SsrState {
            let ElementProps {
                children,
                attributes,
            } = P::into_element_props(self.1);
            let children = IntrinsicChildrenAsElement {
                component_type: self.0,
                children,
            };
            ::frender_ssr::element::html::HtmlElement {
                tag: Cow::Borrowed(C::INTRINSIC_COMPONENT_TAG),
                attributes,
                children: C::wrap_children(children),
            }
            .into_ssr_state()
        }

        type IntoAsyncHtmlChunks = IntrinsicElementEnclosingIter<
            Option<&'static str>,
            <P::Attrs as IntoAsyncAttributeIterator>::IntoAsyncAttributeIterator,
            <P::Children as IntoAsyncStrIterator>::IntoAsyncStrIterator,
        >;

        fn into_async_html_chunks(self) -> Self::IntoAsyncHtmlChunks {
            self.into_async_str_iterator()
        }
    }
}

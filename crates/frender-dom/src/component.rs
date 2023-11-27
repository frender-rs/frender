use frender_ssr::html::{assert::SpaceAndHtmlAttributesOrEmpty, tag::AssertTagName};
pub use props::{ElementProps, IntoElementProps};

mod props;

pub trait HasIntrinsicComponentTag {
    const INTRINSIC_COMPONENT_TAG: &'static str;
    const ASSERT_TAG_NAME: AssertTagName<&'static str>;
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

pub trait IntoSpaceAndHtmlAttributesOrEmpty {
    type SpaceAndHtmlAttributesOrEmpty: SpaceAndHtmlAttributesOrEmpty;
    fn into_space_and_html_attributes_or_empty(self) -> Self::SpaceAndHtmlAttributesOrEmpty;
}

impl IntoSpaceAndHtmlAttributesOrEmpty for () {
    type SpaceAndHtmlAttributesOrEmpty = frender_ssr::Empty;

    fn into_space_and_html_attributes_or_empty(self) -> Self::SpaceAndHtmlAttributesOrEmpty {
        frender_ssr::Empty
    }
}

impl<A: IntoSpaceAndHtmlAttributesOrEmpty, B: IntoSpaceAndHtmlAttributesOrEmpty>
    IntoSpaceAndHtmlAttributesOrEmpty for (A, B)
{
    type SpaceAndHtmlAttributesOrEmpty =
        frender_ssr::Chain<A::SpaceAndHtmlAttributesOrEmpty, B::SpaceAndHtmlAttributesOrEmpty>;

    fn into_space_and_html_attributes_or_empty(self) -> Self::SpaceAndHtmlAttributesOrEmpty {
        frender_ssr::Chain::new(
            self.0.into_space_and_html_attributes_or_empty(),
            self.1.into_space_and_html_attributes_or_empty(),
        )
    }
}

impl<C, P: IntoElementProps> frender_ssr::IntoAsyncStrIterator for IntrinsicElement<C, P>
where
    C: HasIntrinsicComponentTag,
    P::Attrs: IntoSpaceAndHtmlAttributesOrEmpty,
    P::Children: frender_ssr::SsrElement,
{
    type IntoAsyncStrIterator = frender_ssr::html::element::NormalElement<
        AssertTagName<&'static str>,
        <P::Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty,
        <P::Children as frender_ssr::SsrElement>::HtmlChildren,
    >;

    fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
        let ElementProps {
            children,
            attributes,
        } = P::into_element_props(self.1);
        frender_ssr::html::element::NormalElement::new(
            C::ASSERT_TAG_NAME,
            attributes.into_space_and_html_attributes_or_empty(),
            frender_ssr::SsrElement::into_html_children(children),
        )
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
        ElementProps, HasIntrinsicComponentTag, IntoElementProps,
        IntoSpaceAndHtmlAttributesOrEmpty, IntrinsicChildrenAsElement, SsrIntrinsicComponent,
        SsrSupportChildren,
    };

    impl<C: SsrSupportChildren<Children>, Children> Element
        for IntrinsicChildrenAsElement<C, Children>
    {
        type SsrState = C::ChildrenSsrState;

        fn into_ssr_state(self) -> Self::SsrState {
            C::children_into_ssr_state(self.children)
        }

        type HtmlChildren = async_str_iter::empty::Empty;

        fn into_html_children(self) -> Self::HtmlChildren {
            todo!()
        }
    }

    impl<C, P: IntoElementProps> Element for super::IntrinsicElement<C, P>
    where
        // C: super::SsrSupportChildren<P::Children>,
        C: SsrIntrinsicComponent + HasIntrinsicComponentTag,
        P::Attrs: IntoAsyncWritableAttrs,
        P::Children: Element,
        P::Attrs: IntoSpaceAndHtmlAttributesOrEmpty,
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

        type HtmlChildren = frender_ssr::html::element::NormalElement<
            super::AssertTagName<&'static str>,
            <P::Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty,
            <P::Children as frender_ssr::SsrElement>::HtmlChildren,
        >;

        fn into_html_children(self) -> Self::HtmlChildren {
            self.into_async_str_iterator()
        }
    }
}

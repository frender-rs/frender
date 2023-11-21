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

pub struct IntrinsicChildrenAsElement<C, Children> {
    pub component_type: C,
    pub children: Children,
}

mod ssr {
    use std::borrow::Cow;

    use frender_common::write::attrs::IntoAsyncWritableAttrs;
    // use frender_html::dom::component::ElementProps;
    use frender_ssr::Element;

    use super::{
        ElementProps, HasIntrinsicComponentTag, IntoElementProps, IntrinsicChildrenAsElement,
        SsrIntrinsicComponent, SsrSupportChildren,
    };

    impl<C: SsrSupportChildren<Children>, Children> Element
        for IntrinsicChildrenAsElement<C, Children>
    {
        type SsrState = C::ChildrenSsrState;

        fn into_ssr_state(self) -> Self::SsrState {
            C::children_into_ssr_state(self.children)
        }
    }

    impl<C, P: IntoElementProps> Element for super::IntrinsicElement<C, P>
    where
        // C: super::SsrSupportChildren<P::Children>,
        C: SsrIntrinsicComponent + HasIntrinsicComponentTag,
        P::Attrs: IntoAsyncWritableAttrs,
        P::Children: Element,
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
    }
}

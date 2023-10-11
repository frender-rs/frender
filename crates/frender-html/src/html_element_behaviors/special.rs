use frender_html_common::dom_token::DomTokenList;

use super::{define, HtmlElement};

pub trait HtmlElementWithRelList<R: ?Sized>: HtmlElement<R> {
    type RelList<'a>: DomTokenList
    where
        Self: 'a,
        R: 'a;
    fn rel_list<'a>(&'a mut self, renderer: &'a mut R) -> Self::RelList<'a>;
}

mod rel_list {
    use crate::csr::web::{DomTokenList, Node, Renderer};

    frender_common::impl_many!(
        impl
            (
                Generics![R: ?Sized + Renderer],
                Trait![super::HtmlElementWithRelList<R>],
                each_of![
                    //
                    Node<web_sys::HtmlLinkElement>,
                    Node<web_sys::HtmlAnchorElement>,
                    Node<web_sys::HtmlAreaElement>,
                ],
            )
        {
            type RelList<'a> = DomTokenList<R::TryBehavior<'a>>
            where
                Self: 'a,
                R: 'a;
            fn rel_list<'a>(&'a mut self, renderer: &'a mut R) -> Self::RelList<'a> {
                DomTokenList(self.0.rel_list(), renderer.try_behavior())
            }
        }
    );
}

define::define!(
    pub mod prelude {}

    pub trait HtmlElementWithHref {
        extends!(HtmlElementWithRelList);
        special_impl_for!(web_sys::HtmlAnchorElement, web_sys::HtmlAreaElement);

        fn set_download(value: &str);
        fn set_href(value: &str);
        fn set_ping(value: &str);
        fn set_referrer_policy(value: &str);

        fn set_target(value: &str);
    }

    pub trait HtmlElementWithOpen {
        extends!(HtmlElement);
        special_impl_for!(web_sys::HtmlDetailsElement, web_sys::HtmlDialogElement);

        fn set_open(value: bool);
    }

    pub trait HtmlTableChildElement {
        extends!(HtmlElement);
        special_impl_for!(
            web_sys::HtmlTableSectionElement,
            web_sys::HtmlTableRowElement,
            web_sys::HtmlTableColElement,
            web_sys::HtmlTableCellElement,
        );

        #[deprecated]
        fn set_align(value: &str);
        #[deprecated]
        fn set_ch(value: &str);
        #[deprecated]
        fn set_ch_off(value: &str);
        #[deprecated]
        fn set_v_align(value: &str);
    }
);

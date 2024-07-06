pub mod traits {
    use super::web;

    pub trait Node {}

    pub trait Element: Node {
        fn to_element_web(&self) -> Option<web::Element>;
    }

    pub trait HtmlElement: Element {
        fn to_html_element_web(&self) -> Option<web::HtmlElement>;
    }

    #[cfg(feature = "web")]
    mod impl_web {
        use super::*;
        impl Node for web_sys::Element {}
        impl Element for web_sys::Element {
            fn to_element_web(&self) -> Option<web::Element> {
                Some(self.clone().into())
            }
        }

        impl Node for web_sys::HtmlElement {}
        impl Element for web_sys::HtmlElement {
            fn to_element_web(&self) -> Option<web::Element> {
                Some(web_sys::Element::clone(self).into())
            }
        }
        impl HtmlElement for web_sys::HtmlElement {
            fn to_html_element_web(&self) -> Option<web::HtmlElement> {
                Some(web_sys::HtmlElement::clone(self).into())
            }
        }
    }
}

pub mod web {
    use std::ops::Deref;

    use super::traits;

    macro_rules! define_web_sys_wrappers {
        ($vis:vis struct $name:ident {}) => {
            #[derive(Debug, Clone)]
            pub struct $name {
                #[cfg(not(feature = "web"))]
                __: std::convert::Infallible,
                #[cfg(feature = "web")]
                inner: web_sys::$name,
            }

            #[cfg(feature = "web")]
            impl Deref for $name {
                type Target = web_sys::$name;

                fn deref(&self) -> &Self::Target {
                    &self.inner
                }
            }

            #[cfg(feature = "web")]
            impl Into<web_sys::$name> for $name {
                fn into(self) -> web_sys::$name {
                    self.inner
                }
            }

            #[cfg(feature = "web")]
            impl From<web_sys::$name> for $name {
                fn from(value: web_sys::$name) -> Self {
                    Self { inner: value }
                }
            }
        };
        ($($vis:vis struct $name:ident {})*) => {
            $(define_web_sys_wrappers! {
                $vis struct $name {}
            })*
        };
    }

    define_web_sys_wrappers!(
        pub struct Element {}
        pub struct HtmlElement {}
    );

    impl traits::Node for Element {}
    impl traits::Element for Element {
        fn to_element_web(&self) -> Option<Element> {
            Some(self.clone())
        }
    }

    impl traits::Node for HtmlElement {}
    impl traits::Element for HtmlElement {
        fn to_element_web(&self) -> Option<Element> {
            #[cfg(not(feature = "web"))]
            match self.__ {}
            #[cfg(feature = "web")]
            Some(web_sys::Element::clone(self).into())
        }
    }
    impl traits::HtmlElement for HtmlElement {
        fn to_html_element_web(&self) -> Option<self::HtmlElement> {
            Some(self.clone())
        }
    }
}

pub type Element = dyn traits::Element;
pub type HtmlElement = dyn traits::HtmlElement;

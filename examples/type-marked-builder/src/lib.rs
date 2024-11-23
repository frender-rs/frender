#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod _upstream_items {
    pub struct Empty;

    pub trait IsDomTokens {}

    impl IsDomTokens for &'static str {}
}

use _upstream_items::*;

pub mod props_of {
    pub mod Element {
        use crate::prop_markers::Element as prop_markers;
        use crate::Attribute;

        pub struct class<T: crate::IsDomTokens>(pub T);

        impl<T: crate::IsDomTokens> Attribute for class<T> {
            type AttributeMarker = prop_markers::class;
        }
    }

    pub mod HtmlElement {
        pub use super::Element::*;

        pub struct hidden<T>(T);
    }

    pub use HtmlElement as div;

    pub mod HtmlCanvasElement {
        use crate::prop_markers::HtmlCanvasElement as _prop_markers;
        use crate::Attribute;

        pub use super::HtmlElement::*;
        pub struct height<T: Into<Option<u32>>>(pub T);

        impl<T: Into<Option<u32>>> Attribute for height<T> {
            type AttributeMarker = _prop_markers::height;
        }
    }

    pub mod HtmlTableCellElement {
        pub use super::HtmlElement::*;
        use crate::prop_markers::HtmlTableCellElement as _prop_markers;

        use crate::Attribute;

        pub struct height<T: AsRef<str>>(pub T);

        impl<T: AsRef<str>> Attribute for height<T> {
            type AttributeMarker = _prop_markers::height;
        }
    }
}

/// An normal attribute whose render state doesn't need to be pinned.
pub trait Attribute {
    type AttributeMarker;
}

pub struct TypeMarked<M, C, A, P> {
    type_marker: M,
    children: C,
    attributes: A,
    attributes_with_pinned_state: P,
}

impl<M> TypeMarked<M, Empty, (), ()> {
    pub const fn new_empty(type_marker: M) -> Self {
        Self {
            type_marker,
            children: Empty,
            attributes: (),
            attributes_with_pinned_state: (),
        }
    }
}

impl<M, C, A, P> TypeMarked<M, C, A, P> {
    // TODO: Should this be pub?
    fn with_any_attribute_appended<T>(this: Self, attr: T) -> TypeMarked<M, C, (A, T), P> {
        let Self {
            type_marker,
            children,
            attributes,
            attributes_with_pinned_state,
        } = this;
        TypeMarked {
            type_marker,
            children,
            attributes: (attributes, attr),
            attributes_with_pinned_state,
        }
    }

    pub fn with_attribute_appended<T: Attribute>(this: Self, attr: T) -> TypeMarked<M, C, (A, T), P>
    where
        M: AllowAttribute<T::AttributeMarker>,
    {
        Self::with_any_attribute_appended(this, attr)
    }
}

pub mod type_markers {
    use crate::{PropsMarker, TagMarker};

    pub struct Element;
    pub struct HtmlElement;

    pub struct HtmlDivElement;

    impl PropsMarker for HtmlDivElement {}

    pub struct div;

    impl TagMarker for div {
        type PropsMarker = HtmlDivElement;
    }

    pub struct HtmlTableCellElement;
    pub struct td;
    pub struct th;

    pub struct HtmlCanvasElement;
    pub struct canvas;
}

macro_rules! define_mod_Props {
    ($($Props:ident ($($tag:ident),* $(,)?)),* $(,)?) => {
        $(
            pub mod $Props {
                use crate::TypeMarked;

                pub use super::type_markers::$Props as Marker;

                pub type Props<C, A, P> = TypeMarked<super::type_markers::$Props, C, A, P>;
                pub const PROPS: Props<crate::Empty, (), ()> = TypeMarked::new_empty(Marker);

                pub mod tags {
                    $(pub use super::super::$tag;)*
                }
            }

            $(
                // The returned type is zero-sized, so it's likely to be optimized.
                pub const fn $tag() -> $tag::Element<Empty, (), ()> {
                    $tag::ELEMENT
                }

                pub mod $tag {
                    use crate::TypeMarked;

                    pub use super::type_markers::$tag as Marker;
                    pub type Element<C, A, P> = TypeMarked<super::type_markers::$tag, C, A, P>;
                    pub const ELEMENT: Element<crate::Empty, (), ()> = TypeMarked::new_empty(Marker);

                    pub use super::$Props::{self as props, Props, PROPS};
                }
            )*
        )*
    };
}

define_mod_Props!(
    HtmlDivElement(div),
    HtmlCanvasElement(canvas),
    HtmlTableCellElement(td, th),
);

pub trait TagMarker {
    type PropsMarker: PropsMarker;
}
pub trait PropsMarker {}

pub trait AllowAttribute<AttrMarker> {}

impl<TagM: TagMarker, AttrM> AllowAttribute<AttrM> for TagM where
    TagM::PropsMarker: AllowAttribute<AttrM>
{
}

pub mod prop_markers {
    pub mod conflicted_names {
        use crate::{AllowAttributeName, AttributeValue, TypeMarked};

        pub enum height {}

        impl<M: AllowAttributeName<height>, C, A, P> TypeMarked<M, C, A, P> {
            pub fn height<T: AttributeValue<M::AttributeMarker>>(
                self,
                value: T,
            ) -> TypeMarked<M, C, (A, T::Attribute), P> {
                Self::with_attribute_appended(self, T::wrapped_into_attribute(value))
            }
        }
    }

    pub mod Element {
        use crate::{type_markers, AllowAttribute};

        pub enum class {}

        impl AllowAttribute<class> for type_markers::Element {}
    }

    pub mod HtmlElement {
        use crate::{type_markers, AllowAttribute};

        impl<P> AllowAttribute<P> for type_markers::HtmlElement where
            type_markers::Element: AllowAttribute<P>
        {
        }
    }

    pub mod HtmlDivElement {
        use crate::{type_markers, AllowAttribute};

        impl<P> AllowAttribute<P> for type_markers::HtmlDivElement where
            type_markers::HtmlElement: AllowAttribute<P>
        {
        }
    }

    pub mod HtmlCanvasElement {
        use crate::{type_markers, AllowAttribute, AllowAttributeName};

        pub use super::Element::*;
        pub enum height {}

        impl<P> AllowAttribute<P> for type_markers::HtmlCanvasElement where
            type_markers::HtmlElement: AllowAttribute<P>
        {
        }

        impl AllowAttribute<height> for type_markers::HtmlCanvasElement {}
        impl AllowAttributeName<super::conflicted_names::height> for type_markers::HtmlCanvasElement {
            type AttributeMarker = height;
        }

        impl<P> AllowAttributeName<P> for type_markers::canvas
        where
            type_markers::HtmlCanvasElement: AllowAttributeName<P>,
        {
            type AttributeMarker =
                <type_markers::HtmlCanvasElement as AllowAttributeName<P>>::AttributeMarker;
        }

        impl<P> AllowAttribute<P> for type_markers::canvas where
            type_markers::HtmlCanvasElement: AllowAttribute<P>
        {
        }
    }

    pub mod HtmlTableCellElement {
        use crate::{type_markers, AllowAttribute, AllowAttributeName};

        pub use super::Element::*;
        pub enum height {}

        impl AllowAttribute<height> for type_markers::HtmlTableCellElement {}
        impl AllowAttributeName<super::conflicted_names::height> for type_markers::HtmlTableCellElement {
            type AttributeMarker = height;
        }

        impl<P> AllowAttributeName<P> for type_markers::td
        where
            type_markers::HtmlTableCellElement: AllowAttributeName<P>,
        {
            type AttributeMarker =
                <type_markers::HtmlTableCellElement as AllowAttributeName<P>>::AttributeMarker;
        }

        impl<P> AllowAttributeName<P> for type_markers::th
        where
            type_markers::HtmlTableCellElement: AllowAttributeName<P>,
        {
            type AttributeMarker =
                <type_markers::HtmlTableCellElement as AllowAttributeName<P>>::AttributeMarker;
        }

        impl<P> AllowAttribute<P> for type_markers::td where
            type_markers::HtmlTableCellElement: AllowAttribute<P>
        {
        }

        impl<P> AllowAttribute<P> for type_markers::th where
            type_markers::HtmlTableCellElement: AllowAttribute<P>
        {
        }
    }
}

mod props_builders {
    mod Element {
        use crate::prop_markers::Element as prop_markers;
        use crate::props_of::Element as attributes;
        use crate::{AllowAttribute, TypeMarked};

        impl<M: AllowAttribute<prop_markers::class>, C, A, P> TypeMarked<M, C, A, P> {
            pub fn class<T: crate::IsDomTokens>(
                self,
                value: T,
            ) -> TypeMarked<M, C, (A, attributes::class<T>), P> {
                Self::with_attribute_appended(self, attributes::class(value))
            }
        }
    }

    mod HtmlCanvasElement {}

    mod HtmlTableCellElement {}
}

pub trait AttributeValue<AttrMarker> {
    type Attribute: Attribute<AttributeMarker = AttrMarker>;
    fn wrapped_into_attribute(this: Self) -> Self::Attribute;
}

impl<T: Into<Option<u32>>> AttributeValue<prop_markers::HtmlCanvasElement::height> for T {
    type Attribute = props_of::HtmlCanvasElement::height<T>;

    fn wrapped_into_attribute(this: Self) -> Self::Attribute {
        props_of::HtmlCanvasElement::height(this)
    }
}
impl<T: AsRef<str>> AttributeValue<prop_markers::HtmlTableCellElement::height> for T {
    type Attribute = props_of::HtmlTableCellElement::height<T>;

    fn wrapped_into_attribute(this: Self) -> Self::Attribute {
        props_of::HtmlTableCellElement::height(this)
    }
}

pub trait AllowAttributeName<N>: AllowAttribute<Self::AttributeMarker> {
    type AttributeMarker;
}

/// ```
/// # use type_marked_builder as cs;
/// cs::div().class("");
/// ```
///
/// ```compile_fail
/// # use type_marked_builder as cs;
/// cs::div().height("")
/// ```
///
/// ```compile_fail
/// # use type_marked_builder as cs;
/// cs::td().height(1)
/// ```
///
/// ```compile_fail
/// # use type_marked_builder as cs;
/// cs::canvas().height("1px")
/// ```
#[cfg(any(test, doc))]
pub mod doc_tests {}

#[cfg(test)]
#[test]
fn test() {
    let _ = div().class("");

    let _ = canvas().class("").height(1);

    let _ = td().height("2px");
}

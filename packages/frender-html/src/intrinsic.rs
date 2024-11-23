use frender_common::Empty;

// #[cfg(feature = "csr")]
pub use csr_attr_state::AttributeState;

// #[cfg(feature = "csr")]
mod csr_attr_state;
// #[cfg(feature = "csr")]
pub mod csr;

mod ssr;

/// A marker trait which should only be implemented for tag markers or props markers.
pub trait TagOrPropsMarker {}

/// A type marker `M` that makes `Intrinsic<M>` represent
/// *Props* (a props kind and a collection of properties of that kind)
pub trait PropsMarker: TagOrPropsMarker {}

/// A type marker `M` that makes `Intrinsic<M>` represent
/// *Element* (a tag with a collection of properties of its kind)
pub trait TagMarker: TagOrPropsMarker {
    type PropsMarker: PropsMarker;
}

pub trait AllowChildren<C>: TagOrPropsMarker {}

pub trait AllowAttribute<AttrMarker>: TagOrPropsMarker {}
pub trait AllowAttributeWithPinnedState<AttrPinnedMarker>: TagOrPropsMarker {}

pub trait AllowAttributeName<N>: AllowAttribute<Self::AttributeMarker> {
    type AttributeMarker;
}

/// [`TagMarker`] allows all children allowed by its [`PropsMarker`](TagMarker::PropsMarker).
impl<TagM: TagMarker, C> AllowChildren<C> for TagM
where
    TagM::PropsMarker: AllowChildren<C>,
{
    //
}

/// [`TagMarker`] allows all attributes allowed by its [`PropsMarker`](TagMarker::PropsMarker).
impl<TagM: TagMarker, AttrM> AllowAttribute<AttrM> for TagM
where
    TagM::PropsMarker: AllowAttribute<AttrM>,
{
    //
}

/// [`TagMarker`] allows all attributes with pinned state allowed by its [`PropsMarker`](TagMarker::PropsMarker).
impl<TagM: TagMarker, AttrM> AllowAttributeWithPinnedState<AttrM> for TagM
where
    TagM::PropsMarker: AllowAttributeWithPinnedState<AttrM>,
{
    //
}

/// [`TagMarker`] allows all attribute names allowed by its [`PropsMarker`](TagMarker::PropsMarker).
impl<TagM: TagMarker, N> AllowAttributeName<N> for TagM
where
    TagM::PropsMarker: AllowAttributeName<N>,
{
    type AttributeMarker = <TagM::PropsMarker as AllowAttributeName<N>>::AttributeMarker;
}

/// A property is an attribute or an attribute with pinned state.
pub trait Property {
    type PropertyMarker;
}

#[must_use = "Element/Props does nothing unless rendered by a renderer."]
pub struct Intrinsic<TM, C, A, P> {
    type_marker: TM,
    attributes: A,
    // Csr state of `attributes_with_pinned_state` should be pinned.
    //
    // These attributes are only used in csr and ignored in ssr.
    //
    // Currently `attributes_with_pinned_state` only include event listeners.
    attributes_with_pinned_state: P,
    children: C,
}

impl<M> Intrinsic<M, Empty, (), ()> {
    /// Returns an intrinsic element/props with:
    /// - TypeMarker = <argument>
    /// - Children = [`Empty`]
    /// - Attributes = ()
    /// - AttributesWithPinnedState = ()
    pub const fn new_empty(type_marker: M) -> Self {
        Self {
            type_marker,
            attributes: (),
            attributes_with_pinned_state: (),
            children: Empty,
        }
    }
}

impl<M, A, P> Intrinsic<M, Empty, A, P> {
    // TODO: Should with_any* methods be pub?
    fn with_any_children<C>(this: Self, children: C) -> Intrinsic<M, C, A, P> {
        let Intrinsic {
            type_marker,
            attributes,
            attributes_with_pinned_state,
            children: Empty,
        } = this;
        Intrinsic {
            type_marker,
            attributes,
            attributes_with_pinned_state,
            children,
        }
    }

    // TODO: make these builder methods const with #![feature(const_precise_live_drops)] https://github.com/rust-lang/rust/issues/73255
    pub fn children<C>(self, children: C) -> Intrinsic<M, C, A, P>
    where
        M: AllowChildren<C>,
    {
        Self::with_any_children(self, children)
    }
}

impl<M, C, A, P> Intrinsic<M, C, A, P> {
    fn with_map_any_children<NewC>(this: Self, f: impl FnOnce(C) -> NewC) -> Intrinsic<M, NewC, A, P> {
        let Self {
            type_marker,
            attributes,
            attributes_with_pinned_state,
            children,
        } = this;
        Intrinsic {
            type_marker,
            attributes,
            attributes_with_pinned_state,
            children: f(children),
        }
    }

    pub fn with_map_children<NewC>(this: Self, f: impl FnOnce(C) -> NewC) -> Intrinsic<M, NewC, A, P>
    where
        M: AllowChildren<NewC>,
    {
        Self::with_map_any_children(this, f)
    }

    fn with_any_attribute_appended<T>(this: Self, attr: T) -> Intrinsic<M, C, (A, T), P> {
        let Self {
            type_marker,
            children,
            attributes,
            attributes_with_pinned_state,
        } = this;
        Intrinsic {
            type_marker,
            children,
            attributes: (attributes, attr),
            attributes_with_pinned_state,
        }
    }

    fn with_any_attribute_with_pinned_state_appended<T>(this: Self, attr_with_pinned_state: T) -> Intrinsic<M, C, A, (P, T)> {
        let Self {
            type_marker,
            children,
            attributes,
            attributes_with_pinned_state,
        } = this;
        Intrinsic {
            type_marker,
            children,
            attributes,
            attributes_with_pinned_state: (attributes_with_pinned_state, attr_with_pinned_state),
        }
    }

    pub fn with_attribute_appended<T: Property>(this: Self, attr: T) -> Intrinsic<M, C, (A, T), P>
    where
        M: AllowAttribute<T::PropertyMarker>,
    {
        Self::with_any_attribute_appended(this, attr)
    }

    pub fn with_attribute_with_pinned_state_appended<T: Property>(this: Self, attr_with_pinned_state: T) -> Intrinsic<M, C, A, (P, T)>
    where
        M: AllowAttributeWithPinnedState<T::PropertyMarker>,
    {
        Self::with_any_attribute_with_pinned_state_appended(this, attr_with_pinned_state)
    }
}

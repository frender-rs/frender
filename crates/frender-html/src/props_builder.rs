pub trait PropsBuilder: Sized {
    // TODO: IntoSpaceAndAttributes
    type Attributes;
    type Children;
    type EventListeners;
}

pub trait PropsBuilderWithChildren<C>: PropsBuilder<Children = ()> {
    type WithChildren: PropsBuilder<Children = C>;
    fn children(self, children: C) -> Self::WithChildren;
}

pub trait PropsBuilderAppendAnySupportedAttributes: PropsBuilder {
    // TODO: restrict that other associated type are not changed
    type AppendAttributes<A>: PropsBuilder<
        //
        Attributes = (Self::Attributes, A),
        Children = Self::Children,
        EventListeners = Self::EventListeners,
    >;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A>;
}

pub trait PropsBuilderAppendEventListeners: PropsBuilder {
    type AppendEventListeners<EL>: PropsBuilder<
        //
        EventListeners = (Self::EventListeners, EL),
        Attributes = Self::Attributes,
        Children = Self::Children,
    >;
    fn append_event_listeners<EL>(this: Self, el: EL) -> Self::AppendEventListeners<EL>;
}

pub trait PropsBuilderWithValue<V>: PropsBuilder {
    type WithValue;
    /// Alias for [`Self::children`]
    fn value(self, value: V) -> Self::WithValue;
}

pub mod prelude {
    pub use super::{PropsBuilderWithChildren as _, PropsBuilderWithValue as _};
}

mod intrinsic {
    use frender_dom::component::IntrinsicElement;

    use super::*;

    impl<Tag, Props: PropsBuilder> PropsBuilder for IntrinsicElement<Tag, Props> {
        type Attributes = Props::Attributes;
        type Children = Props::Children;
        type EventListeners = Props::EventListeners;
    }

    impl<C, Tag, Props: PropsBuilderWithChildren<C>> PropsBuilderWithChildren<C> for IntrinsicElement<Tag, Props> {
        type WithChildren = IntrinsicElement<Tag, Props::WithChildren>;

        fn children(self, children: C) -> Self::WithChildren {
            IntrinsicElement(self.0, self.1.children(children))
        }
    }

    impl<Tag, Props: PropsBuilderAppendAnySupportedAttributes> PropsBuilderAppendAnySupportedAttributes for IntrinsicElement<Tag, Props> {
        type AppendAttributes<A> = IntrinsicElement<Tag, Props::AppendAttributes<A>>;

        fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
            IntrinsicElement(this.0, Props::append_attributes(this.1, attributes))
        }
    }

    impl<Tag, Props: PropsBuilderAppendEventListeners> PropsBuilderAppendEventListeners for IntrinsicElement<Tag, Props> {
        type AppendEventListeners<EL> = IntrinsicElement<Tag, Props::AppendEventListeners<EL>>;

        fn append_event_listeners<EL>(this: Self, el: EL) -> Self::AppendEventListeners<EL> {
            IntrinsicElement(this.0, Props::append_event_listeners(this.1, el))
        }
    }

    impl<Tag, Props: PropsBuilderWithValue<V>, V> PropsBuilderWithValue<V> for IntrinsicElement<Tag, Props> {
        type WithValue = IntrinsicElement<Tag, Props::WithValue>;

        fn value(self, value: V) -> Self::WithValue {
            IntrinsicElement(self.0, self.1.value(value))
        }
    }
}

pub trait PropsBuilder: Sized {
    // TODO: IntoSpaceAndAttributes
    type Attributes;
    type Children;
    type EventListeners;
}

pub trait PropsBuilderWithChildren<C>: PropsBuilder<Children = crate::Empty> {
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
    fn value(self, value: V) -> Self::WithValue;
}

pub trait PropsBuilderWithType<V>: PropsBuilder {
    type WithType;

    fn r#type(self, t: V) -> Self::WithType;

    /// Alias for [`r#type`](PropsBuilderWithType::r#type).
    fn type_(self, t: V) -> Self::WithType {
        self.r#type(t)
    }
}

pub trait PropsBuilderWithChecked<V>: PropsBuilder {
    type WithChecked;

    fn checked(self, t: V) -> Self::WithChecked;
}

pub mod prelude {
    pub use super::{PropsBuilderWithChecked as _, PropsBuilderWithChildren as _, PropsBuilderWithType as _, PropsBuilderWithValue as _};
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

    impl<Tag, Props: PropsBuilderWithType<V>, V> PropsBuilderWithType<V> for IntrinsicElement<Tag, Props> {
        type WithType = IntrinsicElement<Tag, Props::WithType>;

        fn r#type(self, t: V) -> Self::WithType {
            IntrinsicElement(self.0, self.1.r#type(t))
        }

        fn type_(self, t: V) -> Self::WithType {
            IntrinsicElement(self.0, self.1.type_(t))
        }
    }

    impl<Tag, Props: PropsBuilderWithChecked<V>, V> PropsBuilderWithChecked<V> for IntrinsicElement<Tag, Props> {
        type WithChecked = IntrinsicElement<Tag, Props::WithChecked>;

        fn checked(self, t: V) -> Self::WithChecked {
            IntrinsicElement(self.0, self.1.checked(t))
        }
    }
}

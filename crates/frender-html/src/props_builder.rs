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
    type AppendAttributes<A>: PropsBuilder<Attributes = (Self::Attributes, A)>;
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

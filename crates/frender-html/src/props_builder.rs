pub trait PropsBuilder: Sized {
    // TODO: IntoSpaceAndAttributes
    type Attributes;
    type Children;
}

pub trait PropsBuilderWithChildren<C>: PropsBuilder<Children = ()> {
    type WithChildren: PropsBuilder<Children = C>;
    fn children(self, children: C) -> Self::WithChildren;
}

pub trait PropsBuilderAppendAnySupportedAttributes: PropsBuilder {
    type AppendAttributes<A>: PropsBuilder<Attributes = (Self::Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A>;
}

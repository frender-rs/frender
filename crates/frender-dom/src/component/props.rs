pub trait IntoElementProps {
    type Children;
    type Attrs;

    fn into_element_props(this: Self) -> ElementProps<Self::Children, Self::Attrs>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ElementProps<Children, Attrs> {
    pub children: Children,
    pub attributes: Attrs,
}

impl<Attrs> ElementProps<(), Attrs> {
    #[inline(always)]
    pub fn children<C>(self, children: C) -> ElementProps<C, Attrs> {
        ElementProps {
            children,
            attributes: self.attributes,
        }
    }
}

impl<Children, Attrs> ElementProps<Children, Attrs> {
    #[inline(always)]
    pub fn chain_prop<P>(self, prop: P) -> ElementProps<Children, (Attrs, P)> {
        ElementProps {
            children: self.children,
            attributes: (self.attributes, prop),
        }
    }
}

// pub type ElementPropsEmpty = ElementProps<(), EmptyProps>;

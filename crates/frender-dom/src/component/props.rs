pub trait IntoElementProps {
    type Children;
    type Attributes;
    type EventListeners;

    fn into_element_props(
        this: Self,
    ) -> ElementProps<Self::Children, Self::Attributes, Self::EventListeners>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ElementProps<Children, Attrs, EL> {
    pub children: Children,
    pub attributes: Attrs,
    pub event_listeners: EL,
}

impl<Attrs, EL> ElementProps<(), Attrs, EL> {
    #[inline(always)]
    pub fn children<C>(self, children: C) -> ElementProps<C, Attrs, EL> {
        ElementProps {
            children,
            attributes: self.attributes,
            event_listeners: self.event_listeners,
        }
    }
}

impl<Children, Attrs, EL> ElementProps<Children, Attrs, EL> {
    #[inline(always)]
    pub fn map_children<C>(self, f: impl FnOnce(Children) -> C) -> ElementProps<C, Attrs, EL> {
        let Self {
            children,
            attributes,
            event_listeners,
        } = self;
        ElementProps {
            children: f(children),
            attributes,
            event_listeners,
        }
    }

    #[inline(always)]
    pub fn chain_prop<P>(self, prop: P) -> ElementProps<Children, (Attrs, P), EL> {
        ElementProps {
            children: self.children,
            attributes: (self.attributes, prop),
            event_listeners: self.event_listeners,
        }
    }

    #[inline(always)]
    pub fn chain_event_listener<P>(self, prop: P) -> ElementProps<Children, Attrs, (EL, P)> {
        ElementProps {
            children: self.children,
            attributes: self.attributes,
            event_listeners: (self.event_listeners, prop),
        }
    }
}

// pub type ElementPropsEmpty = ElementProps<(), EmptyProps>;

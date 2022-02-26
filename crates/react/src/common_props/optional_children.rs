pub struct OptionalChildrenProps {
    pub children: Option<crate::Children>,
}

impl OptionalChildrenProps {
    #[inline]
    pub fn children<N: crate::Node>(self, node: Option<N>) -> Self {
        Self {
            children: node.and_then(crate::Node::into_children),
        }
    }
}

impl crate::Props for OptionalChildrenProps {
    type InitialBuilder = OptionalChildrenProps;

    fn init_builder() -> Self::InitialBuilder {
        OptionalChildrenProps { children: None }
    }
}

impl crate::PropsBuilder<OptionalChildrenProps> for OptionalChildrenProps {
    fn build(self) -> OptionalChildrenProps {
        self
    }
}

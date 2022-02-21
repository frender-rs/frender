pub type FragmentProps = crate::OptionalChildrenProps;

pub struct Fragment;

impl crate::UseRenderStatic for Fragment {
    type RenderArg = FragmentProps;
    type RenderOutput = crate::FragmentElement;

    #[inline]
    fn use_render(props: &Self::RenderArg) -> Self::RenderOutput {
        crate::FragmentElement {
            children: props.children.clone(),
            key: None,
        }
    }
}

impl crate::ComponentStatic for Fragment {
    type Props = FragmentProps;
    type Element = crate::FragmentElement;

    #[inline]
    fn create_element(props: Self::Props, key: Option<crate::Key>) -> Self::Element {
        crate::FragmentElement {
            children: props.children,
            key,
        }
    }
}

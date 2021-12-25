pub trait ComponentReturnedElement {}

pub trait Component {
    type ElementType: crate::Element;
    type Props;

    fn render(props: Self::Props) -> Self::ElementType;
}

struct MyTestComponent;

impl Component for MyTestComponent {
    type ElementType = String;
    type Props = ();

    fn render(props: Self::Props) -> Self::ElementType {
        "a".to_string()
    }
}

fn MyTestReactComponent() -> react_sys::Element {
    todo!()
}

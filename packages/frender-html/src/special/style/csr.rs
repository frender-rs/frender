use crate::html::components::style;
use crate::special::parent_only::impl_parent_only;

use crate::csr::component::CsrComponent;
use crate::special::parent_only::RenderInnerTextKind;

use frender_common::reactive_value::ReactiveValueWithKind;
use frender_dom::csr::render_from::str::ValueKindForStr;

impl<Children: ReactiveValueWithKind> CsrComponent<Children> for style::Marker
where
    Children::ReactiveValueKind: ValueKindForStr,
{
    impl_parent_only!(
        type Children = Children;

        type ValueKind = <Children as ReactiveValueWithKind>::ReactiveValueKind;
        type RenderKind = RenderInnerTextKind;

        const into_reactive_value: Children = |children| children;
    );
}

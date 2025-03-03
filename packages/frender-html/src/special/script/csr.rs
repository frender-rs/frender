use frender_dom::script::CsrScriptContent;
use frender_reactive_value::ReactiveValueWithKind;

use crate::{
    csr::component::CsrComponent,
    special::parent_only::{impl_parent_only, RenderInnerTextKind},
};

impl<Children: CsrScriptContent> CsrComponent<Children> for crate::cs::script::Marker {
    impl_parent_only!(
        type Children = Children;

        type ValueKind = <Children::IntoScriptInnerText as ReactiveValueWithKind>::ReactiveValueKind;
        type RenderKind = RenderInnerTextKind;

        const into_reactive_value: Children::IntoScriptInnerText = |children| Children::into_script_inner_text(children);
    );
}

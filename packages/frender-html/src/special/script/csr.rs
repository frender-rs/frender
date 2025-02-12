use frender_common::reactive_value::ReactiveValueWithKind;
use frender_dom::script::IntoScriptContent;

use crate::{
    special::parent_only::{impl_parent_only, RenderInnerTextKind},
    CsrComponent,
};

impl<Children: IntoScriptContent> CsrComponent<Children> for crate::cs::script::Marker {
    impl_parent_only!(
        type Children = Children;

        type ValueKind = <Children::IntoScriptInnerText as ReactiveValueWithKind>::ReactiveValueKind;
        type RenderKind = RenderInnerTextKind;

        const into_reactive_value: Children::IntoScriptInnerText = |children| Children::into_script_inner_text(children);
    );
}

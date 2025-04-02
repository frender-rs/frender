#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
pub mod Node {}
pub mod Element {
    use super::super::prop_markers::Element as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Node::*;
    use crate::intrinsic::Property;
    pub struct children<V>(pub V);
    impl<V> Property for children<V> {
        type PropertyMarker = _prop_markers::children;
    }
    pub struct ref_element<V: FnOnce(&frender_dom::node_ref::Element)>(pub V);
    impl<V: FnOnce(&frender_dom::node_ref::Element)> Property for ref_element<V> {
        type PropertyMarker = _prop_markers::ref_element;
    }
    pub struct class<V: DomTokens::Bounds>(pub V);
    impl<V: DomTokens::Bounds> Property for class<V> {
        type PropertyMarker = _prop_markers::class;
    }
    pub struct id<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for id<V> {
        type PropertyMarker = _prop_markers::id;
    }
    pub struct part<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for part<V> {
        type PropertyMarker = _prop_markers::part;
    }
    pub struct on_cancel<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_cancel<V> {
        type PropertyMarker = _prop_markers::on_cancel;
    }
    pub struct on_error<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_error<V> {
        type PropertyMarker = _prop_markers::on_error;
    }
    pub struct on_scroll<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_scroll<V> {
        type PropertyMarker = _prop_markers::on_scroll;
    }
    pub struct on_security_policy_violation<V: frender_common::MaybeHandleEvent<dyn crate::values::event::SecurityPolicyViolationEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::SecurityPolicyViolationEvent> + 'static> Property for on_security_policy_violation<V> {
        type PropertyMarker = _prop_markers::on_security_policy_violation;
    }
    pub struct on_select<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_select<V> {
        type PropertyMarker = _prop_markers::on_select;
    }
    pub struct on_wheel<V: frender_common::MaybeHandleEvent<dyn crate::values::event::WheelEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::WheelEvent> + 'static> Property for on_wheel<V> {
        type PropertyMarker = _prop_markers::on_wheel;
    }
    pub struct on_copy<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_copy<V> {
        type PropertyMarker = _prop_markers::on_copy;
    }
    pub struct on_cut<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_cut<V> {
        type PropertyMarker = _prop_markers::on_cut;
    }
    pub struct on_paste<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_paste<V> {
        type PropertyMarker = _prop_markers::on_paste;
    }
    pub struct on_composition_end<V: frender_common::MaybeHandleEvent<dyn crate::values::event::CompositionEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::CompositionEvent> + 'static> Property for on_composition_end<V> {
        type PropertyMarker = _prop_markers::on_composition_end;
    }
    pub struct on_composition_start<V: frender_common::MaybeHandleEvent<dyn crate::values::event::CompositionEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::CompositionEvent> + 'static> Property for on_composition_start<V> {
        type PropertyMarker = _prop_markers::on_composition_start;
    }
    pub struct on_composition_update<V: frender_common::MaybeHandleEvent<dyn crate::values::event::CompositionEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::CompositionEvent> + 'static> Property for on_composition_update<V> {
        type PropertyMarker = _prop_markers::on_composition_update;
    }
    pub struct on_blur<V: frender_common::MaybeHandleEvent<dyn crate::values::event::FocusEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::FocusEvent> + 'static> Property for on_blur<V> {
        type PropertyMarker = _prop_markers::on_blur;
    }
    pub struct on_focus<V: frender_common::MaybeHandleEvent<dyn crate::values::event::FocusEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::FocusEvent> + 'static> Property for on_focus<V> {
        type PropertyMarker = _prop_markers::on_focus;
    }
    pub struct on_focus_in<V: frender_common::MaybeHandleEvent<dyn crate::values::event::FocusEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::FocusEvent> + 'static> Property for on_focus_in<V> {
        type PropertyMarker = _prop_markers::on_focus_in;
    }
    pub struct on_focus_out<V: frender_common::MaybeHandleEvent<dyn crate::values::event::FocusEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::FocusEvent> + 'static> Property for on_focus_out<V> {
        type PropertyMarker = _prop_markers::on_focus_out;
    }
    pub struct on_fullscreen_change<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_fullscreen_change<V> {
        type PropertyMarker = _prop_markers::on_fullscreen_change;
    }
    pub struct on_fullscreen_error<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_fullscreen_error<V> {
        type PropertyMarker = _prop_markers::on_fullscreen_error;
    }
    pub struct on_key_down<V: frender_common::MaybeHandleEvent<dyn crate::values::event::KeyboardEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::KeyboardEvent> + 'static> Property for on_key_down<V> {
        type PropertyMarker = _prop_markers::on_key_down;
    }
    pub struct on_key_up<V: frender_common::MaybeHandleEvent<dyn crate::values::event::KeyboardEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::KeyboardEvent> + 'static> Property for on_key_up<V> {
        type PropertyMarker = _prop_markers::on_key_up;
    }
    pub struct on_aux_click<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static> Property for on_aux_click<V> {
        type PropertyMarker = _prop_markers::on_aux_click;
    }
    pub struct on_click<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static> Property for on_click<V> {
        type PropertyMarker = _prop_markers::on_click;
    }
    pub struct on_context_menu<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static> Property for on_context_menu<V> {
        type PropertyMarker = _prop_markers::on_context_menu;
    }
    pub struct on_double_click<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static> Property for on_double_click<V> {
        type PropertyMarker = _prop_markers::on_double_click;
    }
    pub struct on_mouse_down<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static> Property for on_mouse_down<V> {
        type PropertyMarker = _prop_markers::on_mouse_down;
    }
    pub struct on_mouse_enter<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static> Property for on_mouse_enter<V> {
        type PropertyMarker = _prop_markers::on_mouse_enter;
    }
    pub struct on_mouse_leave<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static> Property for on_mouse_leave<V> {
        type PropertyMarker = _prop_markers::on_mouse_leave;
    }
    pub struct on_mouse_move<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static> Property for on_mouse_move<V> {
        type PropertyMarker = _prop_markers::on_mouse_move;
    }
    pub struct on_mouse_out<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static> Property for on_mouse_out<V> {
        type PropertyMarker = _prop_markers::on_mouse_out;
    }
    pub struct on_mouse_over<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static> Property for on_mouse_over<V> {
        type PropertyMarker = _prop_markers::on_mouse_over;
    }
    pub struct on_mouse_up<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static> Property for on_mouse_up<V> {
        type PropertyMarker = _prop_markers::on_mouse_up;
    }
    pub struct on_touch_cancel<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TouchEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TouchEvent> + 'static> Property for on_touch_cancel<V> {
        type PropertyMarker = _prop_markers::on_touch_cancel;
    }
    pub struct on_touch_end<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TouchEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TouchEvent> + 'static> Property for on_touch_end<V> {
        type PropertyMarker = _prop_markers::on_touch_end;
    }
    pub struct on_touch_move<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TouchEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TouchEvent> + 'static> Property for on_touch_move<V> {
        type PropertyMarker = _prop_markers::on_touch_move;
    }
    pub struct on_touch_start<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TouchEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TouchEvent> + 'static> Property for on_touch_start<V> {
        type PropertyMarker = _prop_markers::on_touch_start;
    }
}
pub mod ElementWithHrefAttribute {
    use super::super::prop_markers::ElementWithHrefAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct href<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for href<V> {
        type PropertyMarker = _prop_markers::href;
    }
}
pub mod ElementWithTargetAttribute {
    use super::super::prop_markers::ElementWithTargetAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct target<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for target<V> {
        type PropertyMarker = _prop_markers::target;
    }
}
pub mod ElementWithTypeAttribute {
    use super::super::prop_markers::ElementWithTypeAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct r#type<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for r#type<V> {
        type PropertyMarker = _prop_markers::r#type;
    }
}
pub mod ElementWithCiteAttribute {
    use super::super::prop_markers::ElementWithCiteAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct cite<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for cite<V> {
        type PropertyMarker = _prop_markers::cite;
    }
}
pub mod ElementWithPlaceHolderAttribute {
    use super::super::prop_markers::ElementWithPlaceHolderAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct placeholder<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for placeholder<V> {
        type PropertyMarker = _prop_markers::placeholder;
    }
}
pub mod ElementWithMaxMinLengthAttributes {
    use super::super::prop_markers::ElementWithMaxMinLengthAttributes as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct max_length<V: frender_attr_value::IntoAttrValue<i32>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<i32>> Property for max_length<V> {
        type PropertyMarker = _prop_markers::max_length;
    }
    pub struct min_length<V: frender_attr_value::IntoAttrValue<i32>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<i32>> Property for min_length<V> {
        type PropertyMarker = _prop_markers::min_length;
    }
}
pub mod ElementWithHeightWidthStrAttributes {
    use super::super::prop_markers::ElementWithHeightWidthStrAttributes as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct height<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for height<V> {
        type PropertyMarker = _prop_markers::height;
    }
    pub struct width<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for width<V> {
        type PropertyMarker = _prop_markers::width;
    }
}
pub mod ElementWithHeightWidthU32Attributes {
    use super::super::prop_markers::ElementWithHeightWidthU32Attributes as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct height<V: frender_attr_value::IntoAttrValue<u32>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<u32>> Property for height<V> {
        type PropertyMarker = _prop_markers::height;
    }
    pub struct width<V: frender_attr_value::IntoAttrValue<u32>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<u32>> Property for width<V> {
        type PropertyMarker = _prop_markers::width;
    }
}
pub mod ElementWithMaxF64Attribute {
    use super::super::prop_markers::ElementWithMaxF64Attribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct max<V: frender_attr_value::IntoAttrValue<f64>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<f64>> Property for max<V> {
        type PropertyMarker = _prop_markers::max;
    }
}
pub mod ElementWithValueF64Attribute {
    use super::super::prop_markers::ElementWithValueF64Attribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct value<V: frender_attr_value::IntoAttrValue<f64>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<f64>> Property for value<V> {
        type PropertyMarker = _prop_markers::value;
    }
}
pub mod ElementWithValueStrAttribute {
    use super::super::prop_markers::ElementWithValueStrAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct value<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for value<V> {
        type PropertyMarker = _prop_markers::value;
    }
}
pub mod ElementWithOpenAttribute {
    use super::super::prop_markers::ElementWithOpenAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct open<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for open<V> {
        type PropertyMarker = _prop_markers::open;
    }
}
pub mod ElementWithNameAttribute {
    use super::super::prop_markers::ElementWithNameAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct name<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for name<V> {
        type PropertyMarker = _prop_markers::name;
    }
}
pub mod ElementWithDisabledAttribute {
    use super::super::prop_markers::ElementWithDisabledAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct disabled<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for disabled<V> {
        type PropertyMarker = _prop_markers::disabled;
    }
}
pub mod ElementWithCrossOriginAttribute {
    use super::super::prop_markers::ElementWithCrossOriginAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct cross_origin<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for cross_origin<V> {
        type PropertyMarker = _prop_markers::cross_origin;
    }
}
pub mod ElementWithRelAttribute {
    use super::super::prop_markers::ElementWithRelAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct rel<V: DomTokens::Bounds>(pub V);
    impl<V: DomTokens::Bounds> Property for rel<V> {
        type PropertyMarker = _prop_markers::rel;
    }
}
pub mod ElementWithReferrerPolicyAttribute {
    use super::super::prop_markers::ElementWithReferrerPolicyAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct referrer_policy<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for referrer_policy<V> {
        type PropertyMarker = _prop_markers::referrer_policy;
    }
}
pub mod ElementWithAltAttribute {
    use super::super::prop_markers::ElementWithAltAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct alt<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for alt<V> {
        type PropertyMarker = _prop_markers::alt;
    }
}
pub mod ElementWithLoadingAttribute {
    use super::super::prop_markers::ElementWithLoadingAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct loading<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for loading<V> {
        type PropertyMarker = _prop_markers::loading;
    }
}
pub mod ElementWithAcceptAttribute {
    use super::super::prop_markers::ElementWithAcceptAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct accept<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for accept<V> {
        type PropertyMarker = _prop_markers::accept;
    }
}
pub mod ElementWithAutoCompleteAttribute {
    use super::super::prop_markers::ElementWithAutoCompleteAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct auto_complete<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for auto_complete<V> {
        type PropertyMarker = _prop_markers::auto_complete;
    }
}
pub mod ElementWithAutoCorrectAttribute {
    use super::super::prop_markers::ElementWithAutoCorrectAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct auto_correct<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for auto_correct<V> {
        type PropertyMarker = _prop_markers::auto_correct;
    }
}
pub mod ElementWithFormAttribute {
    use super::super::prop_markers::ElementWithFormAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct form<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for form<V> {
        type PropertyMarker = _prop_markers::form;
    }
}
pub mod ElementWithFormAttributes {
    use super::super::prop_markers::ElementWithFormAttributes as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    #[allow(unused_imports)]
    pub use super::ElementWithFormAttribute::*;
    use crate::intrinsic::Property;
    pub struct form_action<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for form_action<V> {
        type PropertyMarker = _prop_markers::form_action;
    }
    pub struct form_enc_type<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for form_enc_type<V> {
        type PropertyMarker = _prop_markers::form_enc_type;
    }
    pub struct form_method<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for form_method<V> {
        type PropertyMarker = _prop_markers::form_method;
    }
    pub struct form_no_validate<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for form_no_validate<V> {
        type PropertyMarker = _prop_markers::form_no_validate;
    }
    pub struct form_target<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for form_target<V> {
        type PropertyMarker = _prop_markers::form_target;
    }
}
pub mod ElementWithFetchPriorityAttribute {
    use super::super::prop_markers::ElementWithFetchPriorityAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct fetch_priority<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for fetch_priority<V> {
        type PropertyMarker = _prop_markers::fetch_priority;
    }
}
pub mod ElementWithHrefLangAttribute {
    use super::super::prop_markers::ElementWithHrefLangAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    #[allow(unused_imports)]
    pub use super::ElementWithHrefAttribute::*;
    use crate::intrinsic::Property;
    pub struct href_lang<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for href_lang<V> {
        type PropertyMarker = _prop_markers::href_lang;
    }
}
pub mod ElementWithSizesAttribute {
    use super::super::prop_markers::ElementWithSizesAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct sizes<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for sizes<V> {
        type PropertyMarker = _prop_markers::sizes;
    }
}
pub mod ElementWithUseMapAttribute {
    use super::super::prop_markers::ElementWithUseMapAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct use_map<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for use_map<V> {
        type PropertyMarker = _prop_markers::use_map;
    }
}
pub mod ElementWithLabelAttribute {
    use super::super::prop_markers::ElementWithLabelAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct label<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for label<V> {
        type PropertyMarker = _prop_markers::label;
    }
}
pub mod ElementWithForAttribute {
    use super::super::prop_markers::ElementWithForAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct r#for<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for r#for<V> {
        type PropertyMarker = _prop_markers::r#for;
    }
}
pub mod ElementWithIntegrityAttribute {
    use super::super::prop_markers::ElementWithIntegrityAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct integrity<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for integrity<V> {
        type PropertyMarker = _prop_markers::integrity;
    }
}
pub mod ElementWithBlockingAttribute {
    use super::super::prop_markers::ElementWithBlockingAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct blocking<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for blocking<V> {
        type PropertyMarker = _prop_markers::blocking;
    }
}
pub mod ElementWithMultipleAttribute {
    use super::super::prop_markers::ElementWithMultipleAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct multiple<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for multiple<V> {
        type PropertyMarker = _prop_markers::multiple;
    }
}
pub mod ElementWithRequiredAttribute {
    use super::super::prop_markers::ElementWithRequiredAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct required<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for required<V> {
        type PropertyMarker = _prop_markers::required;
    }
}
pub mod ElementWithSizeU32Attribute {
    use super::super::prop_markers::ElementWithSizeU32Attribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct size<V: frender_attr_value::IntoAttrValue<u32>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<u32>> Property for size<V> {
        type PropertyMarker = _prop_markers::size;
    }
}
pub mod ElementWithSrcAttribute {
    use super::super::prop_markers::ElementWithSrcAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct src<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for src<V> {
        type PropertyMarker = _prop_markers::src;
    }
}
pub mod ElementWithSrcsetAttribute {
    use super::super::prop_markers::ElementWithSrcsetAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    #[allow(unused_imports)]
    pub use super::ElementWithSrcAttribute::*;
    use crate::intrinsic::Property;
    pub struct srcset<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for srcset<V> {
        type PropertyMarker = _prop_markers::srcset;
    }
}
pub mod ElementWithBgColorAttribute {
    use super::super::prop_markers::ElementWithBgColorAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct bg_color<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for bg_color<V> {
        type PropertyMarker = _prop_markers::bg_color;
    }
}
pub mod ElementWithAlignAttribute {
    use super::super::prop_markers::ElementWithAlignAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct align<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for align<V> {
        type PropertyMarker = _prop_markers::align;
    }
}
pub mod ElementWithMediaAttribute {
    use super::super::prop_markers::ElementWithMediaAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct media<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for media<V> {
        type PropertyMarker = _prop_markers::media;
    }
}
pub mod ElementWithReadOnlyAttribute {
    use super::super::prop_markers::ElementWithReadOnlyAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct read_only<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for read_only<V> {
        type PropertyMarker = _prop_markers::read_only;
    }
}
pub mod ElementWithDateTimeAttribute {
    use super::super::prop_markers::ElementWithDateTimeAttribute as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct date_time<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for date_time<V> {
        type PropertyMarker = _prop_markers::date_time;
    }
}
pub mod HtmlElement {
    use super::super::prop_markers::HtmlElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::Element::*;
    use crate::intrinsic::Property;
    pub struct ref_html_element<V: FnOnce(&frender_dom::node_ref::HtmlElement)>(pub V);
    impl<V: FnOnce(&frender_dom::node_ref::HtmlElement)> Property for ref_html_element<V> {
        type PropertyMarker = _prop_markers::ref_html_element;
    }
    pub struct access_key<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for access_key<V> {
        type PropertyMarker = _prop_markers::access_key;
    }
    pub struct auto_capitalize<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for auto_capitalize<V> {
        type PropertyMarker = _prop_markers::auto_capitalize;
    }
    pub struct auto_focus<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for auto_focus<V> {
        type PropertyMarker = _prop_markers::auto_focus;
    }
    pub struct content_editable<V: frender_attr_value::IntoAttrValue<AttrKindOfContentEditable>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfContentEditable>> Property for content_editable<V> {
        type PropertyMarker = _prop_markers::content_editable;
    }
    pub struct context_menu<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for context_menu<V> {
        type PropertyMarker = _prop_markers::context_menu;
    }
    pub struct dir<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for dir<V> {
        type PropertyMarker = _prop_markers::dir;
    }
    pub struct draggable<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for draggable<V> {
        type PropertyMarker = _prop_markers::draggable;
    }
    pub struct enter_key_hint<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for enter_key_hint<V> {
        type PropertyMarker = _prop_markers::enter_key_hint;
    }
    pub struct hidden<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for hidden<V> {
        type PropertyMarker = _prop_markers::hidden;
    }
    pub struct inert<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for inert<V> {
        type PropertyMarker = _prop_markers::inert;
    }
    pub struct input_mode<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for input_mode<V> {
        type PropertyMarker = _prop_markers::input_mode;
    }
    pub struct is<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for is<V> {
        type PropertyMarker = _prop_markers::is;
    }
    pub struct item_id<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for item_id<V> {
        type PropertyMarker = _prop_markers::item_id;
    }
    pub struct item_prop<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for item_prop<V> {
        type PropertyMarker = _prop_markers::item_prop;
    }
    pub struct item_ref<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for item_ref<V> {
        type PropertyMarker = _prop_markers::item_ref;
    }
    pub struct item_scope<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for item_scope<V> {
        type PropertyMarker = _prop_markers::item_scope;
    }
    pub struct item_type<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for item_type<V> {
        type PropertyMarker = _prop_markers::item_type;
    }
    pub struct lang<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for lang<V> {
        type PropertyMarker = _prop_markers::lang;
    }
    pub struct nonce<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for nonce<V> {
        type PropertyMarker = _prop_markers::nonce;
    }
    pub struct role<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for role<V> {
        type PropertyMarker = _prop_markers::role;
    }
    pub struct slot<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for slot<V> {
        type PropertyMarker = _prop_markers::slot;
    }
    pub struct spellcheck<V: frender_attr_value::IntoAttrValue<AttrKindOfSpellcheck>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfSpellcheck>> Property for spellcheck<V> {
        type PropertyMarker = _prop_markers::spellcheck;
    }
    pub struct style<V: Style::Bounds>(pub V);
    impl<V: Style::Bounds> Property for style<V> {
        type PropertyMarker = _prop_markers::style;
    }
    pub struct tab_index<V: frender_attr_value::IntoAttrValue<i32>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<i32>> Property for tab_index<V> {
        type PropertyMarker = _prop_markers::tab_index;
    }
    pub struct title<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for title<V> {
        type PropertyMarker = _prop_markers::title;
    }
    pub struct translate<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for translate<V> {
        type PropertyMarker = _prop_markers::translate;
    }
    pub struct virtual_keyboard_policy<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for virtual_keyboard_policy<V> {
        type PropertyMarker = _prop_markers::virtual_keyboard_policy;
    }
    pub struct on_invalid<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_invalid<V> {
        type PropertyMarker = _prop_markers::on_invalid;
    }
    pub struct on_animation_cancel<V: frender_common::MaybeHandleEvent<dyn crate::values::event::AnimationEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::AnimationEvent> + 'static> Property for on_animation_cancel<V> {
        type PropertyMarker = _prop_markers::on_animation_cancel;
    }
    pub struct on_animation_end<V: frender_common::MaybeHandleEvent<dyn crate::values::event::AnimationEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::AnimationEvent> + 'static> Property for on_animation_end<V> {
        type PropertyMarker = _prop_markers::on_animation_end;
    }
    pub struct on_animation_iteration<V: frender_common::MaybeHandleEvent<dyn crate::values::event::AnimationEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::AnimationEvent> + 'static> Property for on_animation_iteration<V> {
        type PropertyMarker = _prop_markers::on_animation_iteration;
    }
    pub struct on_animation_start<V: frender_common::MaybeHandleEvent<dyn crate::values::event::AnimationEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::AnimationEvent> + 'static> Property for on_animation_start<V> {
        type PropertyMarker = _prop_markers::on_animation_start;
    }
    pub struct on_before_input<V: frender_common::MaybeHandleEvent<dyn crate::values::event::InputEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::InputEvent> + 'static> Property for on_before_input<V> {
        type PropertyMarker = _prop_markers::on_before_input;
    }
    pub struct on_input<V: frender_common::MaybeHandleEvent<dyn crate::values::event::InputEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::InputEvent> + 'static> Property for on_input<V> {
        type PropertyMarker = _prop_markers::on_input;
    }
    pub struct on_change<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_change<V> {
        type PropertyMarker = _prop_markers::on_change;
    }
    pub struct on_got_pointer_capture<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static> Property for on_got_pointer_capture<V> {
        type PropertyMarker = _prop_markers::on_got_pointer_capture;
    }
    pub struct on_lost_pointer_capture<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static> Property for on_lost_pointer_capture<V> {
        type PropertyMarker = _prop_markers::on_lost_pointer_capture;
    }
    pub struct on_pointer_cancel<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static> Property for on_pointer_cancel<V> {
        type PropertyMarker = _prop_markers::on_pointer_cancel;
    }
    pub struct on_pointer_down<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static> Property for on_pointer_down<V> {
        type PropertyMarker = _prop_markers::on_pointer_down;
    }
    pub struct on_pointer_enter<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static> Property for on_pointer_enter<V> {
        type PropertyMarker = _prop_markers::on_pointer_enter;
    }
    pub struct on_pointer_leave<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static> Property for on_pointer_leave<V> {
        type PropertyMarker = _prop_markers::on_pointer_leave;
    }
    pub struct on_pointer_move<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static> Property for on_pointer_move<V> {
        type PropertyMarker = _prop_markers::on_pointer_move;
    }
    pub struct on_pointer_out<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static> Property for on_pointer_out<V> {
        type PropertyMarker = _prop_markers::on_pointer_out;
    }
    pub struct on_pointer_over<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static> Property for on_pointer_over<V> {
        type PropertyMarker = _prop_markers::on_pointer_over;
    }
    pub struct on_pointer_up<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static> Property for on_pointer_up<V> {
        type PropertyMarker = _prop_markers::on_pointer_up;
    }
    pub struct on_transition_cancel<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TransitionEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TransitionEvent> + 'static> Property for on_transition_cancel<V> {
        type PropertyMarker = _prop_markers::on_transition_cancel;
    }
    pub struct on_transition_end<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TransitionEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TransitionEvent> + 'static> Property for on_transition_end<V> {
        type PropertyMarker = _prop_markers::on_transition_end;
    }
    pub struct on_transition_run<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TransitionEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TransitionEvent> + 'static> Property for on_transition_run<V> {
        type PropertyMarker = _prop_markers::on_transition_run;
    }
    pub struct on_transition_start<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TransitionEvent> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TransitionEvent> + 'static> Property for on_transition_start<V> {
        type PropertyMarker = _prop_markers::on_transition_start;
    }
    pub struct on_drag<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_drag<V> {
        type PropertyMarker = _prop_markers::on_drag;
    }
    pub struct on_drag_end<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_drag_end<V> {
        type PropertyMarker = _prop_markers::on_drag_end;
    }
    pub struct on_drag_enter<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_drag_enter<V> {
        type PropertyMarker = _prop_markers::on_drag_enter;
    }
    pub struct on_drag_leave<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_drag_leave<V> {
        type PropertyMarker = _prop_markers::on_drag_leave;
    }
    pub struct on_drag_over<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_drag_over<V> {
        type PropertyMarker = _prop_markers::on_drag_over;
    }
    pub struct on_drag_start<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_drag_start<V> {
        type PropertyMarker = _prop_markers::on_drag_start;
    }
    pub struct on_drop<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_drop<V> {
        type PropertyMarker = _prop_markers::on_drop;
    }
}
pub mod HtmlDataListElement {
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlDivElement {
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlDListElement {
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlHeadingElement {
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlHeadElement {
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlHrElement {
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlLegendElement {
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlMenuElement {
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlParagraphElement {
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlPictureElement {
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlPreElement {
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlSpanElement {
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlTemplateElement {
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlTitleElement {
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlElementWithHref {
    use super::super::prop_markers::HtmlElementWithHref as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithHrefAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithReferrerPolicyAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithRelAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithTargetAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct download<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for download<V> {
        type PropertyMarker = _prop_markers::download;
    }
    pub struct ping<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for ping<V> {
        type PropertyMarker = _prop_markers::ping;
    }
}
pub mod HtmlAnchorElement {
    #[allow(unused_imports)]
    pub use super::ElementWithHrefLangAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithTypeAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    #[allow(unused_imports)]
    pub use super::HtmlElementWithHref::*;
}
pub mod HtmlAreaElement {
    use super::super::prop_markers::HtmlAreaElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithAltAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    #[allow(unused_imports)]
    pub use super::HtmlElementWithHref::*;
    use crate::intrinsic::Property;
    pub struct coords<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for coords<V> {
        type PropertyMarker = _prop_markers::coords;
    }
    pub struct shape<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for shape<V> {
        type PropertyMarker = _prop_markers::shape;
    }
}
pub mod HtmlMediaElement {
    use super::super::prop_markers::HtmlMediaElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithCrossOriginAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithSrcAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct auto_play<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for auto_play<V> {
        type PropertyMarker = _prop_markers::auto_play;
    }
    pub struct controls<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for controls<V> {
        type PropertyMarker = _prop_markers::controls;
    }
    pub struct r#loop<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for r#loop<V> {
        type PropertyMarker = _prop_markers::r#loop;
    }
    pub struct muted<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for muted<V> {
        type PropertyMarker = _prop_markers::muted;
    }
    pub struct preload<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for preload<V> {
        type PropertyMarker = _prop_markers::preload;
    }
    pub struct on_abort<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_abort<V> {
        type PropertyMarker = _prop_markers::on_abort;
    }
    pub struct on_can_play<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_can_play<V> {
        type PropertyMarker = _prop_markers::on_can_play;
    }
    pub struct on_can_play_through<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_can_play_through<V> {
        type PropertyMarker = _prop_markers::on_can_play_through;
    }
    pub struct on_duration_change<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_duration_change<V> {
        type PropertyMarker = _prop_markers::on_duration_change;
    }
    pub struct on_emptied<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_emptied<V> {
        type PropertyMarker = _prop_markers::on_emptied;
    }
    pub struct on_ended<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_ended<V> {
        type PropertyMarker = _prop_markers::on_ended;
    }
    pub struct on_loaded_data<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_loaded_data<V> {
        type PropertyMarker = _prop_markers::on_loaded_data;
    }
    pub struct on_loaded_metadata<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_loaded_metadata<V> {
        type PropertyMarker = _prop_markers::on_loaded_metadata;
    }
    pub struct on_load_start<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_load_start<V> {
        type PropertyMarker = _prop_markers::on_load_start;
    }
    pub struct on_pause<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_pause<V> {
        type PropertyMarker = _prop_markers::on_pause;
    }
    pub struct on_play<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_play<V> {
        type PropertyMarker = _prop_markers::on_play;
    }
    pub struct on_playing<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_playing<V> {
        type PropertyMarker = _prop_markers::on_playing;
    }
    pub struct on_progress<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_progress<V> {
        type PropertyMarker = _prop_markers::on_progress;
    }
    pub struct on_rate_change<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_rate_change<V> {
        type PropertyMarker = _prop_markers::on_rate_change;
    }
    pub struct on_resize<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_resize<V> {
        type PropertyMarker = _prop_markers::on_resize;
    }
    pub struct on_seeked<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_seeked<V> {
        type PropertyMarker = _prop_markers::on_seeked;
    }
    pub struct on_seeking<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_seeking<V> {
        type PropertyMarker = _prop_markers::on_seeking;
    }
    pub struct on_stalled<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_stalled<V> {
        type PropertyMarker = _prop_markers::on_stalled;
    }
    pub struct on_suspend<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_suspend<V> {
        type PropertyMarker = _prop_markers::on_suspend;
    }
    pub struct on_time_update<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_time_update<V> {
        type PropertyMarker = _prop_markers::on_time_update;
    }
    pub struct on_volume_change<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_volume_change<V> {
        type PropertyMarker = _prop_markers::on_volume_change;
    }
    pub struct on_waiting<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_waiting<V> {
        type PropertyMarker = _prop_markers::on_waiting;
    }
}
pub mod HtmlBaseElement {
    #[allow(unused_imports)]
    pub use super::ElementWithHrefAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithTargetAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlQuoteElement {
    #[allow(unused_imports)]
    pub use super::ElementWithCiteAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlBodyElement {
    use super::super::prop_markers::HtmlBodyElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct alink<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for alink<V> {
        type PropertyMarker = _prop_markers::alink;
    }
}
pub mod HtmlBrElement {
    use super::super::prop_markers::HtmlBrElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct clear<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for clear<V> {
        type PropertyMarker = _prop_markers::clear;
    }
}
pub mod HtmlButtonElement {
    #[allow(unused_imports)]
    pub use super::ElementWithDisabledAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithFormAttributes::*;
    #[allow(unused_imports)]
    pub use super::ElementWithNameAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithTypeAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithValueStrAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlCanvasElement {
    #[allow(unused_imports)]
    pub use super::ElementWithHeightWidthU32Attributes::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlTableCaptionElement {
    #[allow(unused_imports)]
    pub use super::ElementWithAlignAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlDataElement {
    #[allow(unused_imports)]
    pub use super::ElementWithValueStrAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlModElement {
    #[allow(unused_imports)]
    pub use super::ElementWithCiteAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithDateTimeAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlDetailsElement {
    #[allow(unused_imports)]
    pub use super::ElementWithOpenAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlDialogElement {
    #[allow(unused_imports)]
    pub use super::ElementWithOpenAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlEmbedElement {
    #[allow(unused_imports)]
    pub use super::ElementWithHeightWidthStrAttributes::*;
    #[allow(unused_imports)]
    pub use super::ElementWithSrcAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithTypeAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlFieldSetElement {
    #[allow(unused_imports)]
    pub use super::ElementWithDisabledAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithFormAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithNameAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlFormElement {
    use super::super::prop_markers::HtmlFormElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithAcceptAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithAutoCompleteAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithNameAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithRelAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithTargetAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct accept_charset<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for accept_charset<V> {
        type PropertyMarker = _prop_markers::accept_charset;
    }
    pub struct action<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for action<V> {
        type PropertyMarker = _prop_markers::action;
    }
    pub struct enc_type<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for enc_type<V> {
        type PropertyMarker = _prop_markers::enc_type;
    }
    pub struct method<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for method<V> {
        type PropertyMarker = _prop_markers::method;
    }
    pub struct no_validate<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for no_validate<V> {
        type PropertyMarker = _prop_markers::no_validate;
    }
    pub struct on_form_data<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_form_data<V> {
        type PropertyMarker = _prop_markers::on_form_data;
    }
    pub struct on_reset<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_reset<V> {
        type PropertyMarker = _prop_markers::on_reset;
    }
    pub struct on_submit<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(pub V);
    impl<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static> Property for on_submit<V> {
        type PropertyMarker = _prop_markers::on_submit;
    }
}
pub mod HtmlHtmlElement {
    use super::super::prop_markers::HtmlHtmlElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct xmlns<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for xmlns<V> {
        type PropertyMarker = _prop_markers::xmlns;
    }
}
pub mod HtmlIFrameElement {
    use super::super::prop_markers::HtmlIFrameElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithFetchPriorityAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithHeightWidthStrAttributes::*;
    #[allow(unused_imports)]
    pub use super::ElementWithLoadingAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithNameAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithReferrerPolicyAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithSrcAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct allow<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for allow<V> {
        type PropertyMarker = _prop_markers::allow;
    }
    pub struct allow_fullscreen<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for allow_fullscreen<V> {
        type PropertyMarker = _prop_markers::allow_fullscreen;
    }
    pub struct allow_payment_request<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for allow_payment_request<V> {
        type PropertyMarker = _prop_markers::allow_payment_request;
    }
    pub struct csp<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for csp<V> {
        type PropertyMarker = _prop_markers::csp;
    }
    pub struct sandbox<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for sandbox<V> {
        type PropertyMarker = _prop_markers::sandbox;
    }
    pub struct src_doc<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for src_doc<V> {
        type PropertyMarker = _prop_markers::src_doc;
    }
}
pub mod HtmlImageElement {
    use super::super::prop_markers::HtmlImageElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithAltAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithCrossOriginAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithHeightWidthU32Attributes::*;
    #[allow(unused_imports)]
    pub use super::ElementWithLoadingAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithReferrerPolicyAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithSizesAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithSrcsetAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithUseMapAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct decoding<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for decoding<V> {
        type PropertyMarker = _prop_markers::decoding;
    }
    pub struct element_timing<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for element_timing<V> {
        type PropertyMarker = _prop_markers::element_timing;
    }
    pub struct is_map<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for is_map<V> {
        type PropertyMarker = _prop_markers::is_map;
    }
}
pub mod HtmlInputElement {
    use super::super::prop_markers::HtmlInputElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithAcceptAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithAltAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithAutoCompleteAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithAutoCorrectAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithDisabledAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithFormAttributes::*;
    #[allow(unused_imports)]
    pub use super::ElementWithHeightWidthU32Attributes::*;
    #[allow(unused_imports)]
    pub use super::ElementWithMaxMinLengthAttributes::*;
    #[allow(unused_imports)]
    pub use super::ElementWithMultipleAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithNameAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithPlaceHolderAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithReadOnlyAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithRequiredAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithSizeU32Attribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithSrcAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct capture<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for capture<V> {
        type PropertyMarker = _prop_markers::capture;
    }
    pub struct dirname<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for dirname<V> {
        type PropertyMarker = _prop_markers::dirname;
    }
    pub struct list<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for list<V> {
        type PropertyMarker = _prop_markers::list;
    }
    pub struct max<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for max<V> {
        type PropertyMarker = _prop_markers::max;
    }
    pub struct min<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for min<V> {
        type PropertyMarker = _prop_markers::min;
    }
    pub struct pattern<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for pattern<V> {
        type PropertyMarker = _prop_markers::pattern;
    }
    pub struct step<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for step<V> {
        type PropertyMarker = _prop_markers::step;
    }
}
pub mod HtmlLabelElement {
    #[allow(unused_imports)]
    pub use super::ElementWithForAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlLiElement {
    use super::super::prop_markers::HtmlLiElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct value<V: frender_attr_value::IntoAttrValue<i32>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<i32>> Property for value<V> {
        type PropertyMarker = _prop_markers::value;
    }
}
pub mod HtmlLinkElement {
    use super::super::prop_markers::HtmlLinkElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithBlockingAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithCrossOriginAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithFetchPriorityAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithHrefAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithHrefLangAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithIntegrityAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithMediaAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithReferrerPolicyAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithRelAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithSizesAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithTypeAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct r#as<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for r#as<V> {
        type PropertyMarker = _prop_markers::r#as;
    }
    pub struct image_sizes<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for image_sizes<V> {
        type PropertyMarker = _prop_markers::image_sizes;
    }
    pub struct image_src_set<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for image_src_set<V> {
        type PropertyMarker = _prop_markers::image_src_set;
    }
    pub struct prefetch<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for prefetch<V> {
        type PropertyMarker = _prop_markers::prefetch;
    }
}
pub mod HtmlMapElement {
    #[allow(unused_imports)]
    pub use super::ElementWithNameAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlMetaElement {
    use super::super::prop_markers::HtmlMetaElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithNameAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct charset<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for charset<V> {
        type PropertyMarker = _prop_markers::charset;
    }
    pub struct content<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for content<V> {
        type PropertyMarker = _prop_markers::content;
    }
    pub struct http_equiv<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for http_equiv<V> {
        type PropertyMarker = _prop_markers::http_equiv;
    }
}
pub mod HtmlMeterElement {
    use super::super::prop_markers::HtmlMeterElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithMaxF64Attribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithValueF64Attribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct min<V: frender_attr_value::IntoAttrValue<f64>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<f64>> Property for min<V> {
        type PropertyMarker = _prop_markers::min;
    }
    pub struct low<V: frender_attr_value::IntoAttrValue<f64>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<f64>> Property for low<V> {
        type PropertyMarker = _prop_markers::low;
    }
    pub struct high<V: frender_attr_value::IntoAttrValue<f64>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<f64>> Property for high<V> {
        type PropertyMarker = _prop_markers::high;
    }
    pub struct optimum<V: frender_attr_value::IntoAttrValue<f64>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<f64>> Property for optimum<V> {
        type PropertyMarker = _prop_markers::optimum;
    }
}
pub mod HtmlObjectElement {
    use super::super::prop_markers::HtmlObjectElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithFormAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithHeightWidthStrAttributes::*;
    #[allow(unused_imports)]
    pub use super::ElementWithNameAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithTypeAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithUseMapAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct data<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for data<V> {
        type PropertyMarker = _prop_markers::data;
    }
}
pub mod HtmlOListElement {
    use super::super::prop_markers::HtmlOListElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithTypeAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct reversed<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for reversed<V> {
        type PropertyMarker = _prop_markers::reversed;
    }
    pub struct start<V: frender_attr_value::IntoAttrValue<i32>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<i32>> Property for start<V> {
        type PropertyMarker = _prop_markers::start;
    }
}
pub mod HtmlOptGroupElement {
    #[allow(unused_imports)]
    pub use super::ElementWithDisabledAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithLabelAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlOptionElement {
    use super::super::prop_markers::HtmlOptionElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithDisabledAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithLabelAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithValueStrAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct selected<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for selected<V> {
        type PropertyMarker = _prop_markers::selected;
    }
}
pub mod HtmlOutputElement {
    #[allow(unused_imports)]
    pub use super::ElementWithForAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithFormAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithNameAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlProgressElement {
    #[allow(unused_imports)]
    pub use super::ElementWithMaxF64Attribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithValueF64Attribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlScriptElement {
    use super::super::prop_markers::HtmlScriptElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithBlockingAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithCrossOriginAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithFetchPriorityAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithIntegrityAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithReferrerPolicyAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithSrcAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithTypeAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct children<V: frender_dom::script::ScriptContent>(pub V);
    impl<V: frender_dom::script::ScriptContent> Property for children<V> {
        type PropertyMarker = _prop_markers::children;
    }
    pub struct r#async<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for r#async<V> {
        type PropertyMarker = _prop_markers::r#async;
    }
    pub struct defer<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for defer<V> {
        type PropertyMarker = _prop_markers::defer;
    }
    pub struct no_module<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for no_module<V> {
        type PropertyMarker = _prop_markers::no_module;
    }
}
pub mod HtmlSelectElement {
    #[allow(unused_imports)]
    pub use super::ElementWithAutoCompleteAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithDisabledAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithFormAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithMultipleAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithNameAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithRequiredAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithSizeU32Attribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlSlotElement {
    #[allow(unused_imports)]
    pub use super::ElementWithNameAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlSourceElement {
    #[allow(unused_imports)]
    pub use super::ElementWithHeightWidthU32Attributes::*;
    #[allow(unused_imports)]
    pub use super::ElementWithMediaAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithSizesAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithSrcsetAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithTypeAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlStyleElement {
    #[allow(unused_imports)]
    pub use super::ElementWithBlockingAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithMediaAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithTypeAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlTableElement {
    use super::super::prop_markers::HtmlTableElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithAlignAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithBgColorAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct border<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for border<V> {
        type PropertyMarker = _prop_markers::border;
    }
    pub struct cell_padding<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for cell_padding<V> {
        type PropertyMarker = _prop_markers::cell_padding;
    }
    pub struct cell_spacing<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for cell_spacing<V> {
        type PropertyMarker = _prop_markers::cell_spacing;
    }
    pub struct frame<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for frame<V> {
        type PropertyMarker = _prop_markers::frame;
    }
    pub struct rules<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for rules<V> {
        type PropertyMarker = _prop_markers::rules;
    }
    pub struct summary<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for summary<V> {
        type PropertyMarker = _prop_markers::summary;
    }
    pub struct width<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for width<V> {
        type PropertyMarker = _prop_markers::width;
    }
}
pub mod HtmlTableChildElement {
    use super::super::prop_markers::HtmlTableChildElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithAlignAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithBgColorAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct char<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for char<V> {
        type PropertyMarker = _prop_markers::char;
    }
    pub struct char_off<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for char_off<V> {
        type PropertyMarker = _prop_markers::char_off;
    }
    pub struct v_align<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for v_align<V> {
        type PropertyMarker = _prop_markers::v_align;
    }
}
pub mod HtmlTableSectionElement {
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    #[allow(unused_imports)]
    pub use super::HtmlTableChildElement::*;
}
pub mod HtmlTableRowElement {
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    #[allow(unused_imports)]
    pub use super::HtmlTableChildElement::*;
}
pub mod HtmlTableColElement {
    use super::super::prop_markers::HtmlTableColElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    #[allow(unused_imports)]
    pub use super::HtmlTableChildElement::*;
    use crate::intrinsic::Property;
    pub struct span<V: frender_attr_value::IntoAttrValue<u32>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<u32>> Property for span<V> {
        type PropertyMarker = _prop_markers::span;
    }
    pub struct width<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for width<V> {
        type PropertyMarker = _prop_markers::width;
    }
}
pub mod HtmlTableCellElement {
    use super::super::prop_markers::HtmlTableCellElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithHeightWidthStrAttributes::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    #[allow(unused_imports)]
    pub use super::HtmlTableChildElement::*;
    use crate::intrinsic::Property;
    pub struct col_span<V: frender_attr_value::IntoAttrValue<u32>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<u32>> Property for col_span<V> {
        type PropertyMarker = _prop_markers::col_span;
    }
    pub struct headers<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for headers<V> {
        type PropertyMarker = _prop_markers::headers;
    }
    pub struct row_span<V: frender_attr_value::IntoAttrValue<u32>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<u32>> Property for row_span<V> {
        type PropertyMarker = _prop_markers::row_span;
    }
    pub struct abbr<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for abbr<V> {
        type PropertyMarker = _prop_markers::abbr;
    }
    pub struct axis<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for axis<V> {
        type PropertyMarker = _prop_markers::axis;
    }
    pub struct scope<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for scope<V> {
        type PropertyMarker = _prop_markers::scope;
    }
}
pub mod HtmlTextAreaElement {
    use super::super::prop_markers::HtmlTextAreaElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithAutoCompleteAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithAutoCorrectAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithDisabledAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithFormAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithMaxMinLengthAttributes::*;
    #[allow(unused_imports)]
    pub use super::ElementWithNameAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithPlaceHolderAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithReadOnlyAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithRequiredAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct children<V: TextAreaValue>(pub V);
    impl<V: TextAreaValue> Property for children<V> {
        type PropertyMarker = _prop_markers::children;
    }
    pub struct cols<V: frender_attr_value::IntoAttrValue<u32>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<u32>> Property for cols<V> {
        type PropertyMarker = _prop_markers::cols;
    }
    pub struct rows<V: frender_attr_value::IntoAttrValue<u32>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<u32>> Property for rows<V> {
        type PropertyMarker = _prop_markers::rows;
    }
    pub struct wrap<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for wrap<V> {
        type PropertyMarker = _prop_markers::wrap;
    }
}
pub mod HtmlTimeElement {
    #[allow(unused_imports)]
    pub use super::ElementWithDateTimeAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
}
pub mod HtmlTrackElement {
    use super::super::prop_markers::HtmlTrackElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithLabelAttribute::*;
    #[allow(unused_imports)]
    pub use super::ElementWithSrcAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct default<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for default<V> {
        type PropertyMarker = _prop_markers::default;
    }
    pub struct kind<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for kind<V> {
        type PropertyMarker = _prop_markers::kind;
    }
    pub struct src_lang<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for src_lang<V> {
        type PropertyMarker = _prop_markers::src_lang;
    }
}
pub mod HtmlUListElement {
    use super::super::prop_markers::HtmlUListElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithTypeAttribute::*;
    #[allow(unused_imports)]
    pub use super::HtmlElement::*;
    use crate::intrinsic::Property;
    pub struct compact<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for compact<V> {
        type PropertyMarker = _prop_markers::compact;
    }
}
pub mod HtmlAudioElement {
    #[allow(unused_imports)]
    pub use super::HtmlMediaElement::*;
}
pub mod HtmlVideoElement {
    use super::super::prop_markers::HtmlVideoElement as _prop_markers;
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(unused_imports)]
    pub use super::ElementWithHeightWidthU32Attributes::*;
    #[allow(unused_imports)]
    pub use super::HtmlMediaElement::*;
    use crate::intrinsic::Property;
    pub struct plays_inline<V: frender_attr_value::IntoAttrValue<bool>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<bool>> Property for plays_inline<V> {
        type PropertyMarker = _prop_markers::plays_inline;
    }
    pub struct poster<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(pub V);
    impl<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>> Property for poster<V> {
        type PropertyMarker = _prop_markers::poster;
    }
}

use std::borrow::Cow;

use crate::touch::Touch;

pub trait HasEventTypeName {
    const EVENT_TYPE_NAME: &'static str;
}

macro_rules! type_traits {
    (
        $(#[super_traits($($super_traits:ident),+ $(,)?)])?
        $vis:vis trait $trait_event_type:ident {
            type $type_event:ident;
            $(type $type_event_listener:ident;)?
            $(fn sub_traits() {$(
                $vis_sub_trait:vis trait $sub_trait_event_type:ident
                $sub_trait_details:tt
            )*})?
        }
    ) => {
        $vis use super::$trait_event_type as $type_event;

        crate::expand! {
            while ($($({
                $vis_sub_trait trait $sub_trait_event_type
                $sub_trait_details
            })*)?) {
                prepend(#[super_traits($type_event $($(, $super_traits)+)?)])
                wrap {}
                prepend(type_traits!)
            }
        }
    };
}

macro_rules! type_traits_impl {
    (
        $(#[super_traits($super_trait:ident $(, $super_traits:ident)* $(,)?)])?
        $vis:vis trait $trait_event_type:ident {
            type $type_event:ident;
            $(type $type_event_listener:ident;)?
            $(fn sub_traits() {$(
                $vis_sub_trait:vis trait $sub_trait_event_type:ident
                $sub_trait_details:tt
            )*})?
        }
    ) => {
        #[macro_export]
        macro_rules! $type_event {
            ($for_ty:ty, $element_trait_name:ident, $_type_event:ident, $_type_event_listener:ident) => {
                $(
                    crate::event::type_traits_impl::$super_trait! { $for_ty, $element_trait_name, $_type_event, $_type_event_listener }
                )?

                impl<E: ?Sized, R: ?Sized + crate::RenderHtml> crate::event::type_traits::$type_event<E, R>
                    for $for_ty
                where
                    E: crate::html::behaviors::$element_trait_name<R>,
                {
                    type $type_event = E::$_type_event;
                    $(type $type_event_listener = E::$_type_event_listener;)?
                }
            };
        }

        pub use $type_event;

        crate::expand! {
            while ($($({
                $vis_sub_trait trait $sub_trait_event_type
                $sub_trait_details
            })*)?) {
                prepend(#[super_traits($type_event $(, super_trait $(, $super_traits)*)?)])
                wrap {}
                prepend(type_traits_impl!)
            }
        }
    };
}

macro_rules! define_trait_event_types {
    (
        $(#[super_traits($super_trait:ident $(, $super_traits:ident)* $(,)?)])?
        $vis:vis trait $trait_event_type:ident {
            type $type_event:ident;
            $(type $type_event_listener:ident;)?
            $(fn sub_traits() {$(
                $vis_sub_trait:vis trait $sub_trait_event_type:ident
                $sub_trait_details:tt
            )*})?
        }
    ) => {
        $vis trait $trait_event_type<E: ?Sized, R: ?Sized>
            $(: crate::event::type_traits::$super_trait<E, R, $super_trait = Self::$type_event> )?
        {
            type $type_event: crate::event::$type_event;
            $(type $type_event_listener;)?
        }

        crate::expand! {
            while ($($({
                $vis_sub_trait trait $sub_trait_event_type
                $sub_trait_details
            })*)?) {
                prepend(#[super_traits($type_event $(, $super_trait $(, $super_traits)*)?)])
                wrap {}
                prepend(define_trait_event_types!)
            }
        }
    };
}

macro_rules! define_event_types {
    ($($defs:tt)*) => {
        define_trait_event_types! { $($defs)* }

        pub mod type_traits {
            type_traits! { $($defs)* }
        }
        pub mod type_traits_impl {
            type_traits_impl! { $($defs)* }
        }
    };
}

pub trait Event {
    fn type_(&self) -> Cow<str>;
    fn event_phase(&self) -> u16;
    fn bubbles(&self) -> bool;
    fn cancelable(&self) -> bool;
    fn default_prevented(&self) -> bool;
    fn composed(&self) -> bool;
    fn is_trusted(&self) -> bool;
    fn time_stamp(&self) -> f64;
    fn cancel_bubble(&self) -> bool;

    fn set_cancel_bubble(&self, value: bool);
    fn prevent_default(&self);
    fn stop_immediate_propagation(&self);
    fn stop_propagation(&self);
}

pub trait SecurityPolicyViolationEvent: Event {
    fn document_uri(&self) -> Cow<str>;
    fn referrer(&self) -> Cow<str>;
    fn blocked_uri(&self) -> Cow<str>;
    fn violated_directive(&self) -> Cow<str>;
    fn effective_directive(&self) -> Cow<str>;
    fn original_policy(&self) -> Cow<str>;
    fn source_file(&self) -> Cow<str>;
    fn sample(&self) -> Cow<str>;
    // fn disposition(&self) -> SecurityPolicyViolationEventDisposition; // TODO:
    fn status_code(&self) -> u16;
    fn line_number(&self) -> i32;
    fn column_number(&self) -> i32;
}

/// [`web_sys::UiEvent`]
pub trait UiEvent: Event {
    fn detail(&self) -> i32;
    fn layer_x(&self) -> i32;
    fn layer_y(&self) -> i32;
    fn page_x(&self) -> i32;
    fn page_y(&self) -> i32;
    fn which(&self) -> u32;
    // fn range_parent(&self) -> Option<Node>; // TODO:
    fn range_offset(&self) -> i32;
}

/// [`web_sys::MouseEvent`]
pub trait MouseEvent: UiEvent {
    fn screen_x(&self) -> i32;
    fn screen_y(&self) -> i32;
    fn client_x(&self) -> i32;
    fn client_y(&self) -> i32;
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn offset_x(&self) -> i32;
    fn offset_y(&self) -> i32;
    fn ctrl_key(&self) -> bool;
    fn shift_key(&self) -> bool;
    fn alt_key(&self) -> bool;
    fn meta_key(&self) -> bool;
    fn button(&self) -> i16;
    fn buttons(&self) -> u16;
    // fn related_target(&self) -> Option<EventTarget>; // TODO:
    fn region(&self) -> Option<Cow<str>>;
    fn movement_x(&self) -> i32;
    fn movement_y(&self) -> i32;
    fn get_modifier_state(&self, key_arg: &str) -> bool;
}

/// [`web_sys::WheelEvent`]
pub trait WheelEvent: MouseEvent {
    fn delta_x(&self) -> f64;
    fn delta_y(&self) -> f64;
    fn delta_z(&self) -> f64;
    fn delta_mode(&self) -> u32;
}

/// [`web_sys::CompositionEvent`]
pub trait CompositionEvent: UiEvent {
    fn data(&self) -> Option<Cow<str>>;
    fn locale(&self) -> Cow<str>;
}

/// [`web_sys::FocusEvent`]
pub trait FocusEvent: UiEvent {
    // fn related_target(&self) -> Option<EventTarget>; // TODO:
}

/// [`web_sys::KeyboardEvent`]
pub trait KeyboardEvent: UiEvent {
    fn char_code(&self) -> u32;
    fn key_code(&self) -> u32;
    fn alt_key(&self) -> bool;
    fn ctrl_key(&self) -> bool;
    fn shift_key(&self) -> bool;
    fn meta_key(&self) -> bool;
    fn location(&self) -> u32;
    fn repeat(&self) -> bool;
    fn is_composing(&self) -> bool;
    fn key(&self) -> Cow<str>;
    fn code(&self) -> Cow<str>;
}

/// [`web_sys::TouchEvent`]
pub trait TouchEvent: UiEvent {
    fn touches_to_vec(&self) -> Vec<Touch>;
    fn target_touches_to_vec(&self) -> Vec<Touch>;
    fn changed_touches_to_vec(&self) -> Vec<Touch>;
    fn alt_key(&self) -> bool;
    fn meta_key(&self) -> bool;
    fn ctrl_key(&self) -> bool;
    fn shift_key(&self) -> bool;
}

/// [`web_sys::AnimationEvent`]
pub trait AnimationEvent: Event {
    fn animation_name(&self) -> Cow<str>;
    fn elapsed_time(&self) -> f32;
    fn pseudo_element(&self) -> Cow<str>;
}

/// [`web_sys::InputEvent`]
pub trait InputEvent: UiEvent {
    fn is_composing(&self) -> bool;
    fn input_type(&self) -> Cow<str>;
    fn data(&self) -> Option<Cow<str>>;
    // fn data_transfer(&self) -> Option<DataTransfer>; // TODO:
}

/// [`web_sys::PointerEvent`]
pub trait PointerEvent: MouseEvent {
    fn pointer_id(&self) -> i32;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn pressure(&self) -> f32;
    fn tangential_pressure(&self) -> f32;
    fn tilt_x(&self) -> i32;
    fn tilt_y(&self) -> i32;
    fn twist(&self) -> i32;
    fn pointer_type(&self) -> Cow<str>;
    fn is_primary(&self) -> bool;
    // fn get_coalesced_events(&self) -> ::js_sys::Array; // TODO:
}

/// [`web_sys::TransitionEvent`]
pub trait TransitionEvent: Event {
    fn property_name(&self) -> Cow<str>;
    fn elapsed_time(&self) -> f32;
    fn pseudo_element(&self) -> Cow<str>;
}

define_event_types!(
    pub trait EventType {
        type Event;
        type EventListener;

        fn sub_traits() {
            pub trait SecurityPolicyViolationEventType {
                type SecurityPolicyViolationEvent;
            }

            pub trait UiEventType {
                type UiEvent;

                fn sub_traits() {
                    pub trait MouseEventType {
                        type MouseEvent;

                        fn sub_traits() {
                            pub trait WheelEventType {
                                type WheelEvent;
                            }

                            pub trait PointerEventType {
                                type PointerEvent;
                            }
                        }
                    }

                    pub trait CompositionEventType {
                        type CompositionEvent;
                    }

                    pub trait FocusEventType {
                        type FocusEvent;
                    }

                    pub trait KeyboardEventType {
                        type KeyboardEvent;
                    }

                    pub trait TouchEventType {
                        type TouchEvent;
                    }

                    pub trait InputEventType {
                        type InputEvent;
                    }
                }
            }

            pub trait TransitionEventType {
                type TransitionEvent;
            }

            pub trait AnimationEventType {
                type AnimationEvent;
            }
        }
    }
);

#[cfg(feature = "web")]
mod web {
    use std::borrow::Cow;

    use crate::touch::{Touch, TouchList};

    impl<E: AsRef<web_sys::Event>> super::Event for crate::csr::web::Event<E> {
        fn type_(&self) -> Cow<str> {
            self.0.as_ref().type_().into()
        }
        fn event_phase(&self) -> u16 {
            self.0.as_ref().event_phase()
        }
        fn bubbles(&self) -> bool {
            self.0.as_ref().bubbles()
        }
        fn cancelable(&self) -> bool {
            self.0.as_ref().cancelable()
        }
        fn default_prevented(&self) -> bool {
            self.0.as_ref().default_prevented()
        }
        fn composed(&self) -> bool {
            self.0.as_ref().composed()
        }
        fn is_trusted(&self) -> bool {
            self.0.as_ref().is_trusted()
        }
        fn time_stamp(&self) -> f64 {
            self.0.as_ref().time_stamp()
        }
        fn cancel_bubble(&self) -> bool {
            self.0.as_ref().cancel_bubble()
        }
        fn set_cancel_bubble(&self, value: bool) {
            self.0.as_ref().set_cancel_bubble(value)
        }
        fn prevent_default(&self) {
            self.0.as_ref().prevent_default()
        }
        fn stop_immediate_propagation(&self) {
            self.0.as_ref().stop_immediate_propagation()
        }
        fn stop_propagation(&self) {
            self.0.as_ref().stop_propagation()
        }
    }

    impl<E: AsRef<web_sys::SecurityPolicyViolationEvent> + AsRef<web_sys::Event>>
        super::SecurityPolicyViolationEvent for crate::csr::web::Event<E>
    {
        fn document_uri(&self) -> Cow<str> {
            web_sys::SecurityPolicyViolationEvent::document_uri(self.0.as_ref()).into()
        }
        fn referrer(&self) -> Cow<str> {
            web_sys::SecurityPolicyViolationEvent::referrer(self.0.as_ref()).into()
        }
        fn blocked_uri(&self) -> Cow<str> {
            web_sys::SecurityPolicyViolationEvent::blocked_uri(self.0.as_ref()).into()
        }
        fn violated_directive(&self) -> Cow<str> {
            web_sys::SecurityPolicyViolationEvent::violated_directive(self.0.as_ref()).into()
        }
        fn effective_directive(&self) -> Cow<str> {
            web_sys::SecurityPolicyViolationEvent::effective_directive(self.0.as_ref()).into()
        }
        fn original_policy(&self) -> Cow<str> {
            web_sys::SecurityPolicyViolationEvent::original_policy(self.0.as_ref()).into()
        }
        fn source_file(&self) -> Cow<str> {
            web_sys::SecurityPolicyViolationEvent::source_file(self.0.as_ref()).into()
        }
        fn sample(&self) -> Cow<str> {
            web_sys::SecurityPolicyViolationEvent::sample(self.0.as_ref()).into()
        }
        fn status_code(&self) -> u16 {
            web_sys::SecurityPolicyViolationEvent::status_code(self.0.as_ref())
        }
        fn line_number(&self) -> i32 {
            web_sys::SecurityPolicyViolationEvent::line_number(self.0.as_ref())
        }
        fn column_number(&self) -> i32 {
            web_sys::SecurityPolicyViolationEvent::column_number(self.0.as_ref())
        }
    }

    impl<E: AsRef<web_sys::UiEvent> + AsRef<web_sys::Event>> super::UiEvent
        for crate::csr::web::Event<E>
    {
        fn detail(&self) -> i32 {
            web_sys::UiEvent::detail(self.0.as_ref())
        }
        fn layer_x(&self) -> i32 {
            web_sys::UiEvent::layer_x(self.0.as_ref())
        }
        fn layer_y(&self) -> i32 {
            web_sys::UiEvent::layer_y(self.0.as_ref())
        }
        fn page_x(&self) -> i32 {
            web_sys::UiEvent::page_x(self.0.as_ref())
        }
        fn page_y(&self) -> i32 {
            web_sys::UiEvent::page_y(self.0.as_ref())
        }
        fn which(&self) -> u32 {
            web_sys::UiEvent::which(self.0.as_ref())
        }
        fn range_offset(&self) -> i32 {
            web_sys::UiEvent::range_offset(self.0.as_ref())
        }
    }

    impl<E: AsRef<web_sys::MouseEvent> + AsRef<web_sys::UiEvent> + AsRef<web_sys::Event>>
        super::MouseEvent for crate::csr::web::Event<E>
    {
        fn screen_x(&self) -> i32 {
            web_sys::MouseEvent::screen_x(self.0.as_ref())
        }
        fn screen_y(&self) -> i32 {
            web_sys::MouseEvent::screen_y(self.0.as_ref())
        }
        fn client_x(&self) -> i32 {
            web_sys::MouseEvent::client_x(self.0.as_ref())
        }
        fn client_y(&self) -> i32 {
            web_sys::MouseEvent::client_y(self.0.as_ref())
        }
        fn x(&self) -> i32 {
            web_sys::MouseEvent::x(self.0.as_ref())
        }
        fn y(&self) -> i32 {
            web_sys::MouseEvent::y(self.0.as_ref())
        }
        fn offset_x(&self) -> i32 {
            web_sys::MouseEvent::offset_x(self.0.as_ref())
        }
        fn offset_y(&self) -> i32 {
            web_sys::MouseEvent::offset_y(self.0.as_ref())
        }
        fn ctrl_key(&self) -> bool {
            web_sys::MouseEvent::ctrl_key(self.0.as_ref())
        }
        fn shift_key(&self) -> bool {
            web_sys::MouseEvent::shift_key(self.0.as_ref())
        }
        fn alt_key(&self) -> bool {
            web_sys::MouseEvent::alt_key(self.0.as_ref())
        }
        fn meta_key(&self) -> bool {
            web_sys::MouseEvent::meta_key(self.0.as_ref())
        }
        fn button(&self) -> i16 {
            web_sys::MouseEvent::button(self.0.as_ref())
        }
        fn buttons(&self) -> u16 {
            web_sys::MouseEvent::buttons(self.0.as_ref())
        }
        fn region(&self) -> Option<Cow<str>> {
            web_sys::MouseEvent::region(self.0.as_ref()).map(Cow::Owned)
        }
        fn movement_x(&self) -> i32 {
            web_sys::MouseEvent::movement_x(self.0.as_ref())
        }
        fn movement_y(&self) -> i32 {
            web_sys::MouseEvent::movement_y(self.0.as_ref())
        }
        fn get_modifier_state(&self, key_arg: &str) -> bool {
            web_sys::MouseEvent::get_modifier_state(self.0.as_ref(), key_arg)
        }
    }

    impl<
            E: AsRef<web_sys::WheelEvent>
                + AsRef<web_sys::MouseEvent>
                + AsRef<web_sys::UiEvent>
                + AsRef<web_sys::Event>,
        > super::WheelEvent for crate::csr::web::Event<E>
    {
        fn delta_x(&self) -> f64 {
            web_sys::WheelEvent::delta_x(self.0.as_ref())
        }
        fn delta_y(&self) -> f64 {
            web_sys::WheelEvent::delta_y(self.0.as_ref())
        }
        fn delta_z(&self) -> f64 {
            web_sys::WheelEvent::delta_z(self.0.as_ref())
        }
        fn delta_mode(&self) -> u32 {
            web_sys::WheelEvent::delta_mode(self.0.as_ref())
        }
    }

    impl<E: AsRef<web_sys::CompositionEvent> + AsRef<web_sys::UiEvent> + AsRef<web_sys::Event>>
        super::CompositionEvent for crate::csr::web::Event<E>
    {
        fn data(&self) -> Option<Cow<str>> {
            web_sys::CompositionEvent::data(self.0.as_ref()).map(Cow::Owned)
        }
        fn locale(&self) -> Cow<str> {
            web_sys::CompositionEvent::locale(self.0.as_ref()).into()
        }
    }

    impl<E: AsRef<web_sys::FocusEvent> + AsRef<web_sys::UiEvent> + AsRef<web_sys::Event>>
        super::FocusEvent for crate::csr::web::Event<E>
    {
    }

    impl<E: AsRef<web_sys::KeyboardEvent> + AsRef<web_sys::UiEvent> + AsRef<web_sys::Event>>
        super::KeyboardEvent for crate::csr::web::Event<E>
    {
        fn char_code(&self) -> u32 {
            web_sys::KeyboardEvent::char_code(self.0.as_ref())
        }
        fn key_code(&self) -> u32 {
            web_sys::KeyboardEvent::key_code(self.0.as_ref())
        }
        fn alt_key(&self) -> bool {
            web_sys::KeyboardEvent::alt_key(self.0.as_ref())
        }
        fn ctrl_key(&self) -> bool {
            web_sys::KeyboardEvent::ctrl_key(self.0.as_ref())
        }
        fn shift_key(&self) -> bool {
            web_sys::KeyboardEvent::shift_key(self.0.as_ref())
        }
        fn meta_key(&self) -> bool {
            web_sys::KeyboardEvent::meta_key(self.0.as_ref())
        }
        fn location(&self) -> u32 {
            web_sys::KeyboardEvent::location(self.0.as_ref())
        }
        fn repeat(&self) -> bool {
            web_sys::KeyboardEvent::repeat(self.0.as_ref())
        }
        fn is_composing(&self) -> bool {
            web_sys::KeyboardEvent::is_composing(self.0.as_ref())
        }
        fn key(&self) -> Cow<str> {
            web_sys::KeyboardEvent::key(self.0.as_ref()).into()
        }
        fn code(&self) -> Cow<str> {
            web_sys::KeyboardEvent::code(self.0.as_ref()).into()
        }
    }

    impl<E: AsRef<web_sys::TouchEvent> + AsRef<web_sys::UiEvent> + AsRef<web_sys::Event>>
        super::TouchEvent for crate::csr::web::Event<E>
    {
        fn touches_to_vec(&self) -> Vec<Touch> {
            AsRef::<web_sys::TouchEvent>::as_ref(&self.0)
                .touches()
                .into_vec_of_touches()
        }
        fn target_touches_to_vec(&self) -> Vec<Touch> {
            AsRef::<web_sys::TouchEvent>::as_ref(&self.0)
                .target_touches()
                .into_vec_of_touches()
        }
        fn changed_touches_to_vec(&self) -> Vec<Touch> {
            AsRef::<web_sys::TouchEvent>::as_ref(&self.0)
                .changed_touches()
                .into_vec_of_touches()
        }
        fn alt_key(&self) -> bool {
            web_sys::TouchEvent::alt_key(self.0.as_ref())
        }
        fn meta_key(&self) -> bool {
            web_sys::TouchEvent::meta_key(self.0.as_ref())
        }
        fn ctrl_key(&self) -> bool {
            web_sys::TouchEvent::ctrl_key(self.0.as_ref())
        }
        fn shift_key(&self) -> bool {
            web_sys::TouchEvent::shift_key(self.0.as_ref())
        }
    }

    impl<E: AsRef<web_sys::AnimationEvent> + AsRef<web_sys::Event>> super::AnimationEvent
        for crate::csr::web::Event<E>
    {
        fn animation_name(&self) -> Cow<str> {
            web_sys::AnimationEvent::animation_name(self.0.as_ref()).into()
        }
        fn elapsed_time(&self) -> f32 {
            web_sys::AnimationEvent::elapsed_time(self.0.as_ref())
        }
        fn pseudo_element(&self) -> Cow<str> {
            web_sys::AnimationEvent::pseudo_element(self.0.as_ref()).into()
        }
    }

    impl<E: AsRef<web_sys::InputEvent> + AsRef<web_sys::UiEvent> + AsRef<web_sys::Event>>
        super::InputEvent for crate::csr::web::Event<E>
    {
        fn is_composing(&self) -> bool {
            web_sys::InputEvent::is_composing(self.0.as_ref())
        }
        fn input_type(&self) -> Cow<str> {
            web_sys::InputEvent::input_type(self.0.as_ref()).into()
        }
        fn data(&self) -> Option<Cow<str>> {
            web_sys::InputEvent::data(self.0.as_ref()).map(Cow::Owned)
        }
        // fn data_transfer(&self) -> Option<DataTransfer>; // TODO:
    }

    impl<
            E: AsRef<web_sys::PointerEvent>
                + AsRef<web_sys::MouseEvent>
                + AsRef<web_sys::UiEvent>
                + AsRef<web_sys::Event>,
        > super::PointerEvent for crate::csr::web::Event<E>
    {
        fn pointer_id(&self) -> i32 {
            web_sys::PointerEvent::pointer_id(self.0.as_ref())
        }
        fn width(&self) -> i32 {
            web_sys::PointerEvent::width(self.0.as_ref())
        }
        fn height(&self) -> i32 {
            web_sys::PointerEvent::height(self.0.as_ref())
        }
        fn pressure(&self) -> f32 {
            web_sys::PointerEvent::pressure(self.0.as_ref())
        }
        fn tangential_pressure(&self) -> f32 {
            web_sys::PointerEvent::tangential_pressure(self.0.as_ref())
        }
        fn tilt_x(&self) -> i32 {
            web_sys::PointerEvent::tilt_x(self.0.as_ref())
        }
        fn tilt_y(&self) -> i32 {
            web_sys::PointerEvent::tilt_y(self.0.as_ref())
        }
        fn twist(&self) -> i32 {
            web_sys::PointerEvent::twist(self.0.as_ref())
        }
        fn pointer_type(&self) -> Cow<str> {
            web_sys::PointerEvent::pointer_type(self.0.as_ref()).into()
        }
        fn is_primary(&self) -> bool {
            web_sys::PointerEvent::is_primary(self.0.as_ref())
        }
        // fn get_coalesced_events(&self) -> ::js_sys::Array; // TODO:
    }

    impl<E: AsRef<web_sys::TransitionEvent> + AsRef<web_sys::Event>> super::TransitionEvent
        for crate::csr::web::Event<E>
    {
        fn property_name(&self) -> Cow<str> {
            web_sys::TransitionEvent::property_name(self.0.as_ref()).into()
        }
        fn elapsed_time(&self) -> f32 {
            web_sys::TransitionEvent::elapsed_time(self.0.as_ref())
        }
        fn pseudo_element(&self) -> Cow<str> {
            web_sys::TransitionEvent::pseudo_element(self.0.as_ref()).into()
        }
    }
}

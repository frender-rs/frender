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
                    $crate::event_types::type_traits_impl::$super_trait! { $for_ty, $element_trait_name, $_type_event, $_type_event_listener }
                )?

                impl<E: ?Sized, R: ?Sized + crate::RenderHtml> $crate::event_types::type_traits::$type_event<E, R>
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
            $(: crate::event_types::type_traits::$super_trait<E, R, $super_trait = Self::$type_event> )?
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

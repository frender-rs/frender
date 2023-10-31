pub struct CsrInput<'a, V, E: ?Sized, R: ?Sized> {
    pub this: V,
    pub element: &'a mut E,
    pub renderer: &'a mut R,
    pub attr_name: &'static str,
}

pub struct CsrInputWithUpdater<'a, V, E, RR, U, R> {
    pub this: V,
    pub element: &'a mut E,
    pub renderer: &'a mut RR,
    pub attr_name: &'static str,
    pub update: U,
    pub remove: R,
}

impl<'a, V, E, RR, U, R> CsrInputWithUpdater<'a, V, E, RR, U, R> {
    fn into_value_and_updater(self) -> (V, updater::Updater<'a, E, RR, U, R>) {
        let Self {
            this,
            element,
            renderer,
            attr_name,
            update,
            remove,
        } = self;
        (
            this,
            updater::Updater {
                element,
                renderer,
                attr_name,
                update,
                remove,
            },
        )
    }
}

#[macro_export]
macro_rules! impl_bounds {
    (
        $($wrapper_path_start:ident)? $(:: $wrapper_path:ident)* (
            $($(#$bounds_attrs:tt)+)?
            bounds as $($mod_path_start:ident)?
                $(:: $mod_path:ident)*
                $($(::)? <$($ty:ty),* $(,)?>)?,
            element as $csr_element_ty:ident,
            $attr_name_ident:ident = $attr_name:expr
            $(, $name:ident $fields:tt)*
            $(,)?
        )
    ) => {
        $crate::impl_bounds! {@impl { $($mod_path_start)? $(:: $mod_path)* } {
            wrapper! { $($wrapper_path_start)? $(:: $wrapper_path)* }
            $(bounds_attrs! { $(#$bounds_attrs)+ })?
            bounds!  { $($mod_path_start)? $(:: $mod_path)* }
            bounds_tps! { $($($ty,)*)? }
            csr_element_ty! { $csr_element_ty }
            attr_name! { $attr_name_ident = $attr_name }
        }{
            $($name :: $name $fields)*
        }}
    };
    (
        $($wrapper_path_start:ident)? $(:: $wrapper_path:ident)* (
            $($(#$bounds_attrs:tt)+)?
            bounds as $($mod_path_start:ident)?
                $(:: $mod_path:ident)*
                $($(::)? <$($ty:ty),* $(,)?>)?,
            element as $csr_element_ty:ident
            $(, $name:ident $fields:tt)*
            $(,)?
        )
    ) => {
        $crate::impl_bounds! {@impl { $($mod_path_start)? $(:: $mod_path)* } {
            wrapper! { $($wrapper_path_start)? $(:: $wrapper_path)* }
            $(bounds_attrs! { $(#$bounds_attrs)+ })?
            bounds!  { $($mod_path_start)? $(:: $mod_path)* }
            bounds_tps! { $($($ty,)*)? }
            csr_element_ty! { $csr_element_ty }
        }{
            $($name :: $name $fields)*
        }}
    };
    (@impl {$($bounds:tt)*} $meta:tt {
        csr:: $csr:ident $csr_input:tt
        ssr:: $ssr:ident $ssr_input:tt
    }) => {
        $($bounds)* :: $csr ! {
            meta! $meta
            $csr! $csr_input
        }

        $($bounds)* :: $ssr ! {
            meta! $meta
            $ssr! $ssr_input
        }
    };
    (@impl $bounds:tt $meta:tt {
        csr::$csr:ident $csr_fields:tt
    }) => {
        $crate::impl_bounds! {
            @impl $bounds $meta {
                csr::$csr $csr_fields
                ssr::ssr  {}
            }
        }
    };
    (@impl $bounds:tt $meta:tt {
        ssr::$ssr:ident $ssr_fields:tt
    }) => {
        $crate::impl_bounds! {
            @impl $bounds $meta {
                csr::csr  {}
                ssr::$ssr $ssr_fields
            }
        }
    };
    (@impl $bounds:tt $meta:tt {}) => {
        $crate::impl_bounds! {
            @impl $bounds $meta {
                csr::csr {}
                ssr::ssr {}
            }
        }
    };
}

#[macro_export]
macro_rules! default_impl_csr {
    (
        meta! {
            wrapper! {$($wrapper:tt)*}
            bounds!  {$($bounds:tt)*}
            bounds_tps!  {$($bounds_tp:ty,)*}
            csr_element_ty! { $csr_element_ty:ident }
            $(attr_name! { $attr_name_ident:ident = $attr_name:expr })?
        }
        $csr:ident !{ $($csr_fields:tt)* }
    ) => {
        impl<
            V: $($bounds)*::Bounds::<$($bounds_tp,)*>,
            ET: $crate::html::behavior_type_traits::$csr_element_ty,
        >
            $crate::UpdateElementNonReactive<
                ET
            >
        for $($wrapper)*::<V> {
            type State<Renderer: $crate::RenderHtml> =
                $($wrapper)*::<$($bounds)*::$csr::State![{$($bounds)*}[$($bounds_tp),*][V]]>;

            fn update_element_non_reactive<Renderer: $crate::RenderHtml>(
                Self(this): Self,
                renderer: &mut Renderer,
                element: &mut ET::Node<Renderer>,
                state: &mut Self::State<Renderer>,
            ) {
                // #[allow(unused_imports)]
                use $crate::html::behaviors_prelude::$csr_element_ty::*;

                let element = <ET as $crate::html::behavior_type_traits::$csr_element_ty>::from_identity_mut_root::<Renderer>(element);
                $($bounds)*::$csr::update_with_state($($bounds)*::$csr::Input {
                    this,
                    element,
                    renderer,
                    $($attr_name_ident: $attr_name,)?
                    $($csr_fields)*
                }, &mut state.0)
            }
        }
    };
}

#[macro_export]
macro_rules! default_impl_ssr {
    (
        meta! {
            wrapper! {$($wrapper:tt)*}
            bounds!  {$($bounds:tt)*}
            bounds_tps!  {$($bounds_tp:ty,)*}
            csr_element_ty! { $csr_element_ty:ty }
            attr_name! { $attr_name_ident:ident = $attr_name:expr }
        }
        $ssr:ident !{ $($ssr_fields:tt)* }
    ) => {
        #[cfg(feature = "ssr")]
        impl<V: $($bounds)*::Bounds::<$($bounds_tp,)*>> $crate::__private::IntoAsyncWritableAttrs
            for $($wrapper)*::<V>
        {
            type AsyncWritableAttrs = $($bounds)*::$ssr::Attrs![{$($bounds)*}[$($bounds_tp),*][V]];
            fn into_async_writable_attrs(Self(this): Self) -> Self::AsyncWritableAttrs {
                $($bounds)*::$ssr::into_attrs($($bounds)*::$ssr::Input {
                    this,
                    $attr_name_ident: $attr_name,
                    $($ssr_fields)*
                })
            }
        }
    };
}

#[macro_export]
macro_rules! DefaultCsrState {
    ({$($mod_path:tt)*}[$($($t0:tt)+)?][$($t1:tt)*]) => {
        $($mod_path)* ::csr::State::<$($($t0)*,)? $($t1)*>
    };
}

#[macro_export]
macro_rules! DefaultSsrAttrs {
    ({$($mod_path:tt)*}[$($($t0:tt)+)?][$($t1:tt)*]) => {
        $($mod_path)* ::ssr::Attrs::<$($($t0)*,)? $($t1)*>
    };
}

#[cfg(feature = "ssr")]
pub struct SsrInput<V> {
    pub this: V,
    pub attr_name: &'static str,
}

#[allow(non_snake_case)]
pub mod DomTokens {
    pub use frender_html_common::DomTokens as Bounds;

    #[macro_export]
    macro_rules! __csr_DomTokens {
        (
            meta! {
                wrapper! {$($wrapper:tt)*}
                bounds!  {$($bounds:tt)*}
                bounds_tps!  {$($bounds_tp:ty,)*}
                csr_element_ty! { $csr_element_ty:ident }
                $(attr_name! { $attr_name_ident:ident = $attr_name:expr })?
            }
            $csr:ident !{ $($csr_fields:tt)* }
        ) => {
            impl<
                V: $($bounds)*::Bounds::<$($bounds_tp,)*>,
                ET: $crate::html::behavior_type_traits::$csr_element_ty,
            >
                $crate::UpdateElementNonReactive<
                    ET
                >
            for $($wrapper)*::<V> {
                type State<Renderer: $crate::RenderHtml> =
                    $($wrapper)*::<$($bounds)*::$csr::State![{$($bounds)*}[$($bounds_tp),*][V]]>;

                fn update_element_non_reactive<Renderer: $crate::RenderHtml>(
                    Self(this): Self,
                    renderer: &mut Renderer,
                    element: &mut ET::Node<Renderer>,
                    state: &mut Self::State<Renderer>,
                ) {
                    use $crate::html::behaviors_prelude::$csr_element_ty::*;

                    let element = <ET as $crate::html::behavior_type_traits::$csr_element_ty>::from_identity_mut_root::<Renderer>(element);

                    let input = $($bounds)*::$csr::Input {
                        this,
                        element,
                        renderer,
                        $($attr_name_ident: $attr_name,)?
                        $($csr_fields)*
                    };

                    let state = &mut state.0;

                    let mut dom_token_list = (input.get_mut_dom_token_list)(input.element, input.renderer);
                    V::update_with_state(input.this, &mut dom_token_list, state)
                }
            }
        };
    }

    pub use crate::default_impl_ssr as ssr;
    pub use __csr_DomTokens as csr;

    #[cfg(feature = "csr")]
    pub mod csr {
        use frender_html_common::DomTokens;

        pub use crate::DefaultCsrState as State;

        pub type State<V> = <V as DomTokens>::UpdateWithState;

        pub struct Input<'a, V, E: ?Sized, RR: ?Sized, F> {
            pub this: V,
            pub element: &'a mut E,
            pub renderer: &'a mut RR,
            pub attr_name: &'static str,
            pub get_mut_dom_token_list: F,
        }
    }

    #[cfg(feature = "ssr")]
    pub mod ssr {
        use frender_common::write::attrs::AsyncWritableAttrValueStr;
        use frender_html_common::DomTokens;

        pub use super::super::SsrInput as Input;
        pub use crate::DefaultSsrAttrs as Attrs;

        pub type Attrs<V> = Option<
            frender_ssr::element::html::simple::AttrPairStr<
                <V as DomTokens>::AsyncWritableDomTokens,
            >,
        >;

        pub fn into_attrs<V: DomTokens>(Input { this, attr_name }: Input<V>) -> Attrs<V> {
            V::maybe_into_async_writable_dom_tokens(this).map(|value| {
                frender_ssr::element::html::simple::AttrPairStr::new_from_str(
                    attr_name,
                    AsyncWritableAttrValueStr::new(value),
                )
            })
        }
    }
}

mod updater {
    pub(super) struct Updater<'a, E, RR, U, R> {
        pub(super) element: &'a mut E,
        pub(super) renderer: &'a mut RR,
        pub(super) attr_name: &'static str,
        pub(super) update: U,
        pub(super) remove: R,
    }

    impl<
            'a,
            VT: ?Sized,
            E,
            RR,
            U: FnOnce(&mut E, &mut RR, &'static str, &VT),
            R: FnOnce(&mut E, &mut RR, &'static str),
        > frender_html_common::ValueUpdater<VT> for Updater<'a, E, RR, U, R>
    {
        fn update(mut self, value: &VT) {
            (self.update)(&mut self.element, &mut self.renderer, self.attr_name, value)
        }

        fn remove(mut self) {
            (self.remove)(&mut self.element, &mut self.renderer, self.attr_name)
        }
    }
}

#[allow(non_snake_case)]
pub mod MaybeValue {
    pub use frender_html_common::MaybeUpdateValueWithState as Bounds;

    pub use crate::default_impl_csr as csr;
    pub use crate::default_impl_ssr as ssr;

    pub mod csr {
        use std::borrow::Cow;

        use frender_html_common::{MaybeUpdateValueWithState, ValueType};

        pub use super::super::CsrInputWithUpdater as Input;
        pub use crate::DefaultCsrState as State;

        pub type State<VT, V> = <V as MaybeUpdateValueWithState<VT>>::UpdateWithState;

        pub fn update_with_state<
            VT: ?Sized + ValueType,
            V: MaybeUpdateValueWithState<VT>,
            E,
            RR,
            U: FnOnce(&mut E, &mut RR, &'static str, &VT),
            R: FnOnce(&mut E, &mut RR, &'static str),
        >(
            input: Input<V, E, RR, U, R>,
            state: &mut State<VT, V>,
        ) {
            let (this, updater) = input.into_value_and_updater();
            V::update_with_state(this, state, updater)
        }

        // TODO: remove and use UpdateElementAttribute
        pub trait AsAttributeValue {
            fn as_attribute_value(&self) -> Cow<str>;
        }

        impl AsAttributeValue for str {
            fn as_attribute_value(&self) -> Cow<str> {
                Cow::Borrowed(self)
            }
        }

        impl AsAttributeValue for bool {
            fn as_attribute_value(&self) -> Cow<str> {
                debug_assert!(*self);
                Cow::Borrowed("")
            }
        }

        impl AsAttributeValue for u32 {
            fn as_attribute_value(&self) -> Cow<str> {
                Cow::Owned(self.to_string())
            }
        }

        pub fn default_update<
            V: ?Sized + AsAttributeValue,
            E: ?Sized + crate::renderer::node_behaviors::Element<RR>,
            RR: ?Sized,
        >(
            element: &mut E,
            renderer: &mut RR,
            prop_name: &'static str,
            value: &V,
        ) {
            element.set_attribute(renderer, prop_name, &value.as_attribute_value())
        }

        pub fn default_remove<
            E: ?Sized + crate::renderer::node_behaviors::Element<RR>,
            RR: ?Sized,
        >(
            element: &mut E,
            renderer: &mut RR,
            prop_name: &'static str,
        ) {
            element.remove_attribute(renderer, prop_name)
        }
    }

    pub mod ssr {
        use frender_html_common::{MaybeUpdateValueWithState, ValueType};
        use frender_ssr::element::html::simple::AttrPair;

        pub use super::super::SsrInput as Input;

        pub type Attrs<VT, V> = Option<AttrPair<<V as MaybeUpdateValueWithState<VT>>::AttrValue>>;

        pub use crate::DefaultSsrAttrs as Attrs;

        pub fn into_attrs<VT: ?Sized + ValueType, V: MaybeUpdateValueWithState<VT>>(
            Input { this, attr_name }: Input<V>,
        ) -> Attrs<VT, V>
        where
            VT::SupportIntoAttrValue: Default,
        {
            V::maybe_into_attr_value(this, Default::default())
                .map(|value| AttrPair::new_from_str(attr_name, value))
        }
    }
}

#[allow(non_snake_case)]
pub mod MaybeHandleEvent {
    pub use frender_events::MaybeHandleEvent as Bounds;

    #[macro_export]
    macro_rules! __impl_csr_MaybeHandleEvent {
        (
            meta! {
                wrapper! {$($wrapper:tt)*}
                bounds_attrs! { #[event($($bounds_tp:tt)*)] }
                bounds!  {$($bounds:tt)*}
                bounds_tps!  {}
                csr_element_ty! { $csr_element_ty:ident }
                $(attr_name! { $attr_name_ident:ident = $attr_name:expr })?
            }
            $csr:ident !{ $($csr_fields:tt)* }
        ) => {
            impl<
                V: $($bounds)*::Bounds::<dyn $($bounds_tp)* ::Event>,
                ET: $crate::html::behavior_type_traits::$csr_element_ty,
            >
                $crate::UpdateElementNonReactive<
                    ET
                >
            for $($wrapper)*::<V> {
                type State<Renderer: $crate::RenderHtml> =
                    $($wrapper)*::<$($bounds)*::$csr::State<
                        dyn $($bounds_tp)* ::Event,
                        V,
                        $($bounds_tp)*::EventListenerOf<ET::$csr_element_ty<Renderer>, Renderer>
                    >>;

                fn update_element_non_reactive<Renderer: $crate::RenderHtml>(
                    Self(this): Self,
                    renderer: &mut Renderer,
                    element: &mut ET::Node<Renderer>,
                    state: &mut Self::State<Renderer>,
                ) {
                    use $crate::html::behaviors_prelude::$csr_element_ty::*;

                    let element = <ET as $crate::html::behavior_type_traits::$csr_element_ty>::from_identity_mut_root::<Renderer>(element);
                    $($bounds)*::$csr::update_with_state($($bounds)*::$csr::Input {
                        this,
                        element,
                        renderer,
                        // $($attr_name_ident: $attr_name,)?
                        new_event_state: $($bounds_tp)* ::on,
                        $($csr_fields)*
                    }, &mut state.0)
                }
            }
        };
    }

    pub use __impl_csr_MaybeHandleEvent as csr;

    #[cfg(feature = "csr")]
    pub mod csr {
        use frender_events::{callable::StatedEvent, EventListener, MaybeHandleEvent, NewFromRef};
        use wasm_bindgen::JsCast;

        pub struct Input<'a, V, E: ?Sized, R: ?Sized, F> {
            pub this: V,
            pub element: &'a mut E,
            pub renderer: &'a mut R,
            pub new_event_state: F,
        }

        pub type State<Ev, V, S> =
            <V as frender_events::callable::gat::MaybeHandleEvent<Ev>>::UpdateWithState<S>;

        pub fn update_with_state<
            Ev: ?Sized,
            V: MaybeHandleEvent<Ev>,
            S,
            E: ?Sized + crate::renderer::node_behaviors::Element<RR>,
            RR: ?Sized,
            F: FnOnce(&mut E, &mut RR, &<V as MaybeHandleEvent<Ev>>::StaticCloneCallable) -> S,
        >(
            Input {
                this,
                element,
                renderer,
                new_event_state,
            }: Input<V, E, RR, F>,
            state: &mut State<Ev, V, S>,
        ) {
            V::update_with_state(this, state, |callable| {
                new_event_state(element, renderer, callable)
            })
        }
    }

    #[macro_export]
    macro_rules! __impl_ssr_MaybeHandleEvent {
        (
            meta! {
                wrapper! {$($wrapper:tt)*}
                bounds_attrs! { #[event($($bounds_tp:tt)*)] }
                bounds!  {$($bounds:tt)*}
                bounds_tps!  {}
                csr_element_ty! { $csr_element_ty:ty }
                $(attr_name! { $attr_name_ident:ident = $attr_name:expr })?
            }
            $ssr:ident !{ $($ssr_fields:tt)* }
        ) => {
            #[cfg(feature = "ssr")]
            impl<V: $($bounds)*::Bounds::<dyn $($bounds_tp)* ::Event>> $crate::__private::IntoAsyncWritableAttrs
                for $($wrapper)*::<V>
            {
                type AsyncWritableAttrs = ();
                fn into_async_writable_attrs(_: Self) {}
            }
        };
    }

    pub use __impl_ssr_MaybeHandleEvent as ssr;
}

#[allow(non_snake_case)]
pub mod MaybeContentEditable {
    pub use crate::default_impl_csr as csr;
    pub use crate::default_impl_ssr as ssr;
    pub use frender_html_common::MaybeContentEditable as Bounds;

    pub mod csr {
        use frender_html_common::MaybeContentEditable;

        pub use super::super::CsrInputWithUpdater as Input;
        pub use crate::DefaultCsrState as State;

        pub type State<V> = <V as MaybeContentEditable>::UpdateWithState;

        pub fn update_with_state<
            V: MaybeContentEditable,
            E,
            RR,
            U: FnOnce(&mut E, &mut RR, &'static str, &str),
            R: FnOnce(&mut E, &mut RR, &'static str),
        >(
            input: Input<V, E, RR, U, R>,
            state: &mut State<V>,
        ) {
            let (this, updater) = input.into_value_and_updater();

            V::update_with_state(this, updater, state)
        }

        pub use super::super::MaybeValue::csr::default_remove;
    }

    pub mod ssr {
        use frender_common::write::attrs::AsyncWritableAttrValueStr;
        use frender_html_common::MaybeContentEditable;
        use frender_ssr::element::html::simple::AttrPairStr;

        pub use super::super::SsrInput as Input;
        pub use crate::DefaultSsrAttrs as Attrs;

        pub type Attrs<V> = Option<AttrPairStr<<V as MaybeContentEditable>::AttrValueStr>>;

        pub fn into_attrs<V: MaybeContentEditable>(
            Input { this, attr_name }: Input<V>,
        ) -> Attrs<V> {
            V::maybe_into_attr_value_str(this).map(|value| {
                AttrPairStr::new_from_str(attr_name, AsyncWritableAttrValueStr::new(value))
            })
        }
    }
}

#[allow(non_snake_case)]
pub mod Css {
    pub use frender_html_common::Css as Bounds;

    pub use crate::default_impl_csr as csr;
    pub use crate::default_impl_ssr as ssr;

    pub mod csr {
        pub use super::super::CsrInput as Input;
        pub use crate::DefaultCsrState as State;

        pub type State<V> = Option<V>;

        pub fn update_with_state<V, E, R>(input: Input<V, E, R>, state: &mut State<V>) {}
    }

    pub mod ssr {
        use frender_common::write::str::StrWriting;

        pub use super::super::SsrInput as Input;
        pub use crate::css_type_attrs as Attrs;

        pub type Attrs =
            Option<frender_ssr::element::html::simple::AttrPairStr<StrWriting<String>>>;

        #[macro_export]
        macro_rules! css_type_attrs {
            ({$($mod_path:tt)*}[$($($t0:tt)+)?][$($t1:tt)*]) => {
                $crate::impl_bounds::Css::ssr::Attrs
            };
        }

        pub fn into_attrs<V>(input: Input<V>) -> Attrs {
            None
        }
    }
}
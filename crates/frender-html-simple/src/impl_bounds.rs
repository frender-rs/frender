pub struct Data<'a, V, E, R> {
    pub this: V,
    pub element: &'a mut E,
    pub renderer: &'a mut R,
    pub attr_name: &'static str,
}

#[cfg(feature = "csr")]
pub struct CsrInput<'a, 'ctx, V, E> {
    pub this: V,
    pub element: &'a E,
    pub children_ctx: &'a mut frender_csr::CsrContext<'ctx>,
    pub attr_name: &'static str,
}

pub struct DataWithUpdater<'a, V, E, RR, U, R> {
    pub this: V,
    pub element: &'a mut E,
    pub renderer: &'a mut RR,
    pub attr_name: &'static str,
    pub update: U,
    pub remove: R,
}

#[cfg(feature = "csr")]
pub struct CsrInputWithUpdater<'a, 'ctx, V, E, U, R> {
    pub this: V,
    pub element: &'a E,
    pub children_ctx: &'a mut frender_csr::CsrContext<'ctx>,
    pub attr_name: &'static str,
    pub update: U,
    pub remove: R,
}

#[macro_export]
macro_rules! impl_bounds {
    (
        $($wrapper_path_start:ident)? $(:: $wrapper_path:ident)* (
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
            bounds!  { $($mod_path_start)? $(:: $mod_path)* }
            bounds_tps! { $($($ty,)*)? }
            csr_element_ty! { $csr_element_ty }
            attr_name! { $attr_name_ident = $attr_name }
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
            attr_name! { $attr_name_ident:ident = $attr_name:expr }
        }
        $csr:ident !{ $($csr_fields:tt)* }
    ) => {
        impl<
            V: $($bounds)*::Bounds::<$($bounds_tp,)*>,
            ET: $crate::__private::frender_html::element_type_traits::$csr_element_ty,
        >
            $crate::__private::frender_html::UpdateElementNonReactive<
                ET
            >
        for $($wrapper)*::<V> {
            type State = $($wrapper)*::<$($bounds)*::$csr::State![{$($bounds)*}[$($bounds_tp),*][V]]>;

            fn update_element_non_reactive<Renderer: $crate::__private::frender_html::RenderHtml>(
                Self(this): Self,
                renderer: &mut Renderer,
                element: &mut $crate::__private::frender_html::ElementOfType<ET, Renderer>,
                state: &mut Self::State,
            ) {
                let element: &mut <ET as $crate::__private::frender_html::element_type_traits::$csr_element_ty>::$csr_element_ty::<Renderer> = $crate::__private::frender_html::Identity::from_identity_mut(element);
                $($bounds)*::$csr::update_with_state($($bounds)*::$csr::Data {
                    this,
                    element,
                    renderer,
                    attr_name: $attr_name,
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

    pub use crate::default_impl_csr as csr;
    pub use crate::default_impl_ssr as ssr;

    #[cfg(feature = "csr")]
    pub mod csr {
        use frender_csr::{web_sys, CsrContext};
        use frender_html_common::DomTokens;

        pub use crate::DefaultCsrState as State;

        pub type State<V> = <V as DomTokens>::UpdateState;

        pub struct Input<'a, 'ctx, V: DomTokens, El, G: FnOnce(&El) -> web_sys::DomTokenList> {
            pub this: V,
            pub element: &'a El,
            pub children_ctx: &'a mut CsrContext<'ctx>,
            pub attr_name: &'static str,
            pub get_dom_token: G,
        }

        pub fn initialize<V: DomTokens, El, G: FnOnce(&El) -> web_sys::DomTokenList>(
            Input {
                this,
                element,
                get_dom_token,
                ..
            }: Input<V, El, G>,
        ) -> State<V> {
            V::update_dom_token_list_and_initialize_state(this, &get_dom_token(element))
        }

        pub fn update<V: DomTokens, El, G: FnOnce(&El) -> web_sys::DomTokenList>(
            Input {
                this,
                element,
                get_dom_token,
                ..
            }: Input<V, El, G>,
            state: &mut State<V>,
        ) {
            V::update_dom_token_list(this, &get_dom_token(element), state)
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

#[allow(non_snake_case)]
pub mod MaybeValue {
    pub use frender_html_common::MaybeUpdateValueWithState as Bounds;

    pub use crate::default_impl_csr as csr;
    pub use crate::default_impl_ssr as ssr;

    #[cfg(feature = "csr")]
    pub mod csr {
        use frender_csr::{props::UpdateElementAttribute, web_sys, CsrContext};
        use frender_html_common::{MaybeUpdateValueWithState, ValueType};

        pub use super::super::CsrInputWithUpdater as Input;
        pub use super::super::DataWithUpdater as Data;
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
            Data {
                this,
                element,
                renderer,
                attr_name,
                update,
                remove,
            }: Data<V, E, RR, U, R>,
            state: &mut State<VT, V>,
        ) {
            struct Updater<'a, E, RR, U, R> {
                element: &'a mut E,
                renderer: &'a mut RR,
                attr_name: &'static str,
                update: U,
                remove: R,
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

            V::update_with_state(
                this,
                state,
                Updater {
                    element,
                    renderer,
                    attr_name,
                    update,
                    remove,
                },
            )
        }

        pub fn default_update<E: AsRef<web_sys::Element>, V: ?Sized + UpdateElementAttribute>(
            element: &E,
            prop_name: &'static str,
            v: &V,
        ) {
            V::update_element_attribute(v, element.as_ref(), prop_name)
        }

        pub fn default_remove<E: AsRef<web_sys::Element>>(element: &E, prop_name: &'static str) {
            element.as_ref().remove_attribute(prop_name).unwrap()
        }

        pub fn remove<
            E: ?Sized + frender_html::renderer::node_behaviors::Element<RR>,
            RR: ?Sized,
        >(
            element: &mut E,
            renderer: &mut RR,
            prop_name: &'static str,
        ) {
            element.remove_attribute(renderer, prop_name)
        }
    }

    #[cfg(feature = "ssr")]
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

    pub use crate::default_impl_csr as csr;

    #[cfg(feature = "csr")]
    pub mod csr {
        use frender_csr::wasm_bindgen::JsCast;
        use frender_csr::web_sys;
        use frender_events::{callable::StatedEvent, EventListener, MaybeHandleEvent, NewFromRef};

        pub use super::super::CsrInput as Input;
        pub use crate::DefaultCsrState as State;

        pub type State<E, V> = <V as frender_events::callable::MaybeHandleEvent<E>>::State;

        pub fn initialize<
            Ev: ?Sized + StatedEvent<State = EventListener>,
            V: MaybeHandleEvent<Ev>,
            El: AsRef<web_sys::EventTarget>,
        >(
            Input {
                this,
                element,
                attr_name,
                ..
            }: Input<V, El>,
        ) -> State<Ev, V>
        where
            Ev: NewFromRef,
            Ev::Target: JsCast,
        {
            V::initialize_handle_event_state(this, |callable| {
                EventListener::new(element.as_ref(), attr_name, callable.clone())
            })
        }

        pub fn update<
            Ev: ?Sized + StatedEvent<State = EventListener>,
            V: MaybeHandleEvent<Ev>,
            El: AsRef<web_sys::EventTarget>,
        >(
            Input {
                this,
                element,
                attr_name,
                ..
            }: Input<V, El>,
            state: &mut State<Ev, V>,
        ) where
            Ev: NewFromRef,
            Ev::Target: JsCast,
        {
            V::update_handle_event_state(this, state, |callable| {
                EventListener::new(element.as_ref(), attr_name, callable.clone())
            })
        }
    }

    #[macro_export]
    macro_rules! __impl_EventListener_ssr {
        (
            meta! {
                wrapper! {$($wrapper:tt)*}
                bounds!  {$($bounds:tt)*}
                bounds_tps!  {$($bounds_tp:ty,)*}
                csr_element_ty! { $csr_element_ty:ty }
                attr_name! { $attr_name:expr }
            }
            $ssr:ident !{ $($ssr_fields:tt)* }
        ) => {
            #[cfg(feature = "ssr")]
            impl<V: $($bounds)*::Bounds::<$($bounds_tp,)*>> $crate::__private::IntoAsyncWritableAttrs
                for $($wrapper)*::<V>
            {
                type AsyncWritableAttrs = ();
                fn into_async_writable_attrs(_: Self) {}
            }
        };
    }

    pub use __impl_EventListener_ssr as ssr;
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

        pub type State<V> = <V as MaybeContentEditable>::State;

        pub fn initialize<
            V: MaybeContentEditable,
            E,
            U: FnOnce(&str, &E, &'static str),
            R: FnOnce(&E, &'static str),
        >(
            Input {
                this,
                element,
                children_ctx: _,
                attr_name,
                update,
                remove,
            }: Input<V, E, U, R>,
        ) -> State<V> {
            V::initialize(
                this,
                |v| update(v, element, attr_name),
                || remove(element, attr_name),
            )
        }

        pub fn update<
            V: MaybeContentEditable,
            E,
            U: FnOnce(&str, &E, &'static str),
            R: FnOnce(&E, &'static str),
        >(
            Input {
                this,
                element,
                children_ctx: _,
                attr_name,
                update,
                remove,
            }: Input<V, E, U, R>,
            state: &mut State<V>,
        ) {
            V::update(
                this,
                |v| update(v, element, attr_name),
                || remove(element, attr_name),
                state,
            )
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

        pub type State<V> = V;

        pub fn initialize<V, E>(input: Input<V, E>) -> State<V> {
            input.this
        }
        pub fn update<V, E>(input: Input<V, E>, state: &mut State<V>) {}
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

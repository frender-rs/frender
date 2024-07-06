pub struct CsrInput<'a, V, E: ?Sized, R: ?Sized> {
    pub this: V,
    pub element: &'a mut E,
    pub renderer: &'a mut R,
    pub attr_name: &'static str,
}

pub struct CsrInputWithUpdater<'a, V, E: ?Sized, RR: ?Sized, U, R> {
    pub this: V,
    pub element: &'a mut E,
    pub renderer: &'a mut RR,
    pub attr_name: &'static str,
    pub update: U,
    pub remove: R,
}

impl<'a, V, E: ?Sized, RR: ?Sized, U, R> CsrInputWithUpdater<'a, V, E, RR, U, R> {
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
            csr_state_wrapper($($csr_state_wrapper:tt)+),
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
            csr_state_wrapper! { $($csr_state_wrapper)+ }
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
            csr_state_wrapper! {$($csr_state_wrapper:tt)*}
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
            $crate::UpdateNodeNonReactive<
                ET
            >
        for $($wrapper)*::<V> {
            type State<Renderer: $crate::RenderHtml + ?::core::marker::Sized> =
                $($csr_state_wrapper)*::<$($bounds)*::$csr::State![{$($bounds)*}[$($bounds_tp),*][V]]>;

            fn update_node_non_reactive<Renderer: $crate::RenderHtml + ?::core::marker::Sized>(
                Self(this): Self,
                renderer: &mut Renderer,
                element: &mut ET::NodeOfBehaviorType<Renderer>,
                state: &mut Self::State<Renderer>,
            ) {
                #[allow(unused_imports)]
                use $crate::html::behaviors_prelude::$csr_element_ty::*;

                let element = <<ET as $crate::html::behavior_type_traits::$csr_element_ty>::$csr_element_ty<Renderer> as frender_common::convert::FromMut<_>>::from_mut(element);
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
            csr_state_wrapper! {$($csr_state_wrapper:tt)*}
            bounds!  {$($bounds:tt)*}
            bounds_tps!  {$($bounds_tp:ty,)*}
            csr_element_ty! { $csr_element_ty:ty }
            attr_name! { $attr_name_ident:ident = $attr_name:expr }
        }
        $ssr:ident !{ $($ssr_fields:tt)* }
    ) => {
        impl<V: $($bounds)*::Bounds::<$($bounds_tp,)*>> $crate::dom::component::IntoSpaceAndHtmlAttributesOrEmpty
            for $($wrapper)*::<V>
        {
            type SpaceAndHtmlAttributesOrEmpty = ::async_str_iter::option::IterOption<::frender_ssr::html::attr::SpaceAndHtmlAttribute<
                ::frender_ssr::html::attr::AssertSpaceAndHtmlAttributeName<&'static str>,
                $($bounds)*::$ssr::Haevoe![{$($bounds)*}[$($bounds_tp),*][V]],
            >>;

            fn into_space_and_html_attributes_or_empty(self) -> Self::SpaceAndHtmlAttributesOrEmpty {
                const SPACE_AND_ATTR_NAME: ::frender_ssr::html::attr::AssertSpaceAndHtmlAttributeName<&'static str> =
                    ::frender_ssr::html::attr::AssertSpaceAndHtmlAttributeName::new_from_str(
                        ::core::concat!(" ", $attr_name)
                    );

                ::async_str_iter::IntoAsyncStrIterator::into_async_str_iterator(
                    $($bounds)*::$ssr::maybe_into_haevoe(self.0, $($ssr_fields)*).map(|v| ::frender_ssr::html::attr::SpaceAndHtmlAttribute(
                        SPACE_AND_ATTR_NAME,
                        v,
                    ))
                )
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

#[macro_export]
macro_rules! DefaultSsrHaevoe {
    ({$($mod_path:tt)*}[$($($t0:tt)+)?][$($t1:tt)*]) => {
        $($mod_path)* ::ssr::Haevoe::<$($($t0)*,)? $($t1)*>
    };
}

#[allow(non_snake_case)]
pub mod DomTokens {
    pub use frender_html_common::DomTokens as Bounds;

    #[macro_export]
    macro_rules! __csr_DomTokens {
        (
            meta! {
                wrapper! {$($wrapper:tt)*}
                csr_state_wrapper! {$($csr_state_wrapper:tt)*}
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
                $crate::UpdateNodeNonReactive<
                    ET
                >
            for $($wrapper)*::<V> {
                type State<Renderer: $crate::RenderHtml + ?::core::marker::Sized> =
                    $($csr_state_wrapper)*::<$($bounds)*::$csr::State![{$($bounds)*}[$($bounds_tp),*][V]]>;

                fn update_node_non_reactive<Renderer: $crate::RenderHtml + ?::core::marker::Sized>(
                    Self(this): Self,
                    renderer: &mut Renderer,
                    element: &mut ET::NodeOfBehaviorType<Renderer>,
                    state: &mut Self::State<Renderer>,
                ) {
                    #[allow(unused_imports)]
                    use $crate::html::behaviors_prelude::$csr_element_ty::*;

                    let element = <<ET as $crate::html::behavior_type_traits::$csr_element_ty>::$csr_element_ty<Renderer> as frender_common::convert::FromMut<_>>::from_mut(element);

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

    pub mod ssr {
        use frender_html_common::DomTokens;

        pub use crate::DefaultSsrHaevoe as Haevoe;

        pub type Haevoe<V> = frender_ssr::html::attr_value::AttrEqValue<<V as DomTokens>::DomTokensIntoAsyncStrIter>;

        pub fn maybe_into_haevoe<V: DomTokens>(this: V) -> Option<Haevoe<V>> {
            Some(Haevoe::<V>::new(V::dom_tokens_into_async_str_iter(this)))
        }
    }
}

mod updater {
    use frender_html_common::ValueKind;

    pub(super) struct Updater<'a, E: ?Sized, RR: ?Sized, U, R> {
        pub(super) element: &'a mut E,
        pub(super) renderer: &'a mut RR,
        pub(super) attr_name: &'static str,
        pub(super) update: U,
        pub(super) remove: R,
    }

    impl<
            //
            'a,
            VT: ?Sized + ValueKind,
            E: ?Sized,
            RR: ?Sized,
            U: FnOnce(&mut E, &mut RR, &'static str, VT::Value<'_>),
            R: FnOnce(&mut E, &mut RR, &'static str),
        > frender_html_common::ValueUpdater<VT> for Updater<'a, E, RR, U, R>
    {
        fn update(mut self, value: VT::Value<'_>) {
            (self.update)(&mut self.element, &mut self.renderer, self.attr_name, value)
        }

        fn remove(mut self) {
            (self.remove)(&mut self.element, &mut self.renderer, self.attr_name)
        }
    }
}

#[allow(non_snake_case)]
pub mod MaybeValue {
    pub use frender_html_common::attr::MaybeAttrValue as Bounds;

    pub use crate::default_impl_csr as csr;
    pub use crate::default_impl_ssr as ssr;

    pub mod csr {
        use frender_html_common::{MaybeValue, ValueKind};

        pub use super::super::CsrInputWithUpdater as Input;
        pub use crate::DefaultCsrState as State;

        pub type State<VT, V> = <V as MaybeValue<VT>>::UpdateWithState;

        pub fn update_with_state<
            //
            VT: ?Sized + ValueKind,
            V: MaybeValue<VT>,
            E,
            RR: ?Sized,
            U: FnOnce(&mut E, &mut RR, &'static str, VT::Value<'_>),
            R: FnOnce(&mut E, &mut RR, &'static str),
        >(
            input: Input<V, E, RR, U, R>,
            state: &mut State<VT, V>,
        ) {
            let (this, updater) = input.into_value_and_updater();
            V::update_with_state(this, state, updater)
        }
    }

    pub mod ssr {
        use frender_html_common::{attr::MaybeIntoHtmlAttributeValue, ValueKind};

        pub use crate::DefaultSsrHaevoe as Haevoe;

        pub type Haevoe<VT, V> = <V as MaybeIntoHtmlAttributeValue<VT>>::HtmlAttributeValue;

        pub fn maybe_into_haevoe<VT: ?Sized + ValueKind, V: MaybeIntoHtmlAttributeValue<VT>>(this: V) -> Option<Haevoe<VT, V>> {
            V::maybe_into_html_attribute_value(this)
        }
    }
}

#[allow(non_snake_case)]
pub mod MaybeHandleEvent {
    #[macro_export]
    macro_rules! __impl_csr_MaybeHandleEvent {
        (
            meta! {
                wrapper! {$($wrapper:tt)*}
                csr_state_wrapper! {$($csr_state_wrapper:tt)*}
                bounds_attrs! { #[event($($bounds_tp:tt)*)] }
                bounds!  {$($bounds:tt)*}
                bounds_tps!  {}
                csr_element_ty! { $csr_element_ty:ident }
                $(attr_name! { $attr_name_ident:ident = $attr_name:expr })?
            }
            $csr:ident !{ $($csr_fields:tt)* }
        ) => {
            impl<
                H: frender_dom::HandleEvent<dyn $($bounds_tp)* ::Event> + 'static,
                V: frender_dom::MaybeHandleEvent<dyn $($bounds_tp)* ::Event, HandleEvent = H> + 'static,
                ET: $crate::html::behavior_type_traits::$csr_element_ty,
            >
                $crate::UpdateNodeNonReactive<
                    ET
                >
            for $($wrapper)*::<V> {
                type State<Renderer: $crate::RenderHtml + ?::core::marker::Sized> =
                    $($bounds_tp)*::UnpinnedEventListenerOf<
                        ET::$csr_element_ty<Renderer>,
                        Renderer,
                        H,
                    >;

                fn update_node_non_reactive<Renderer: $crate::RenderHtml + ?::core::marker::Sized>(
                    Self(this): Self,
                    renderer: &mut Renderer,
                    element: &mut ET::NodeOfBehaviorType<Renderer>,
                    state: &mut Self::State<Renderer>,
                ) {
                    #[allow(unused_imports)]
                    use $crate::html::behaviors_prelude::$csr_element_ty::*;

                    let element = <<ET as $crate::html::behavior_type_traits::$csr_element_ty>::$csr_element_ty::<Renderer> as frender_common::convert::FromMut<_>>::from_mut(element);

                    if let Some(this) = this.into() {
                        frender_dom::RegisterOrUpdate::register_or_update(
                            std::pin::Pin::new(state),
                            element,
                            renderer,
                            this,
                        )
                    } else {
                        *state = Default::default()
                    }
                }
            }

            impl<
                H: frender_dom::HandleEvent<dyn $($bounds_tp)* ::Event> + 'static,
                V: frender_dom::MaybeHandleEvent<dyn $($bounds_tp)* ::Event, HandleEvent = H> + 'static,
                ET: $crate::html::behavior_type_traits::$csr_element_ty,
            >
                $crate::UpdateNodeNonReactivePinned<
                    ET
                >
            for $($wrapper)*::<V> {
                type StatePinned<Renderer: $crate::RenderHtml + ?::core::marker::Sized> =
                    $($bounds_tp)*::UnpinnedEventListenerOf<
                        ET::$csr_element_ty<Renderer>,
                        Renderer,
                        H,
                    >;

                fn update_node_non_reactive_pinned<Renderer: $crate::RenderHtml + ?::core::marker::Sized>(
                    Self(this): Self,
                    renderer: &mut Renderer,
                    element: &mut ET::NodeOfBehaviorType<Renderer>,
                    mut state: std::pin::Pin<&mut Self::StatePinned<Renderer>>,
                ) {
                    #[allow(unused_imports)]
                    use $crate::html::behaviors_prelude::$csr_element_ty::*;

                    let element = <<ET as $crate::html::behavior_type_traits::$csr_element_ty>::$csr_element_ty::<Renderer> as frender_common::convert::FromMut<_>>::from_mut(element);

                    if let Some(this) = this.into() {
                        frender_dom::RegisterOrUpdate::register_or_update(
                            state,
                            element,
                            renderer,
                            this,
                        )
                    } else {
                        state.set(Default::default())
                    }
                }
            }
        };
    }

    pub use __impl_csr_MaybeHandleEvent as csr;

    #[macro_export]
    macro_rules! __impl_ssr_MaybeHandleEvent {
        (
            meta! {
                wrapper! {$($wrapper:tt)*}
                csr_state_wrapper! {$($csr_state_wrapper:tt)*}
                bounds_attrs! { #[event($($bounds_tp:tt)*)] }
                bounds!  {$($bounds:tt)*}
                bounds_tps!  {}
                csr_element_ty! { $csr_element_ty:ty }
                $(attr_name! { $attr_name_ident:ident = $attr_name:expr })?
            }
            $ssr:ident !{ $($ssr_fields:tt)* }
        ) => {
            impl<
                V: frender_dom::MaybeHandleEvent<dyn $($bounds_tp)* ::Event> + 'static,
            > $crate::dom::component::IntoSpaceAndHtmlAttributesOrEmpty
                for $($wrapper)*::<V>
            {
                type SpaceAndHtmlAttributesOrEmpty = ::async_str_iter::empty::Empty;

                fn into_space_and_html_attributes_or_empty(self) -> Self::SpaceAndHtmlAttributesOrEmpty {
                    ::async_str_iter::empty::Empty
                }
            }
        };
    }

    pub use __impl_ssr_MaybeHandleEvent as ssr;
}

#[allow(non_snake_case)]
pub mod SetRef {
    pub use FnOnceSetRef as Bounds;

    /// A trait alias for `FnOnce(&dyn frender_dom::node_ref::traits::_)`
    pub trait FnOnceSetRef<N: ?Sized + frender_dom::node_ref::traits::Node>: FnOnce(&N) {}

    impl<N: ?Sized + frender_dom::node_ref::traits::Node, F: FnOnce(&N)> FnOnceSetRef<N> for F {}

    #[macro_export]
    macro_rules! __Ref_csr {
        (
            meta! {
                wrapper! {$($wrapper:tt)*}
                csr_state_wrapper! {$($csr_state_wrapper:tt)*}
                bounds!  {$($bounds:tt)*}
                bounds_tps!  {$($bounds_tps:ty),* $(,)?}
                csr_element_ty! { $csr_element_ty:ident }
                $(attr_name! { $attr_name_ident:ident = $attr_name:expr })?
            }
            $csr:ident !{ $($csr_fields:tt)* }
        ) => {
            impl<
                V: FnOnce($(&$bounds_tps),*),
                ET: $crate::html::behavior_type_traits::$csr_element_ty,
            >
                $crate::UpdateNodeNonReactive<
                    ET
                >
            for $($wrapper)*::<V> {
                type State<Renderer: $crate::RenderHtml + ?::core::marker::Sized> = ();

                fn update_node_non_reactive<Renderer: $crate::RenderHtml + ?::core::marker::Sized>(
                    Self(this): Self,
                    _: &mut Renderer,
                    element: &mut ET::NodeOfBehaviorType<Renderer>,
                    (): &mut Self::State<Renderer>,
                ) {
                    let element = <<ET as $crate::html::behavior_type_traits::$csr_element_ty>::$csr_element_ty::<Renderer> as frender_common::convert::FromMut<_>>::from_mut(element);

                    this(frender_dom::behaviors::$csr_element_ty::as_node_ref(element))
                }
            }
        };
    }

    #[macro_export]
    macro_rules! __Ref_ssr {
        (
            meta! {
                wrapper! {$($wrapper:tt)*}
                csr_state_wrapper! {$($csr_state_wrapper:tt)*}
                bounds!  {$($bounds:tt)*}
                bounds_tps!  {$($bounds_tps:ty),* $(,)?}
                csr_element_ty! { $csr_element_ty:ident }
                $(attr_name! { $attr_name_ident:ident = $attr_name:expr })?
            }
            $ssr:ident !{ $($ssr_fields:tt)* }
        ) => {
            impl<
                V: FnOnce($(&$bounds_tps),*),
            > $crate::dom::component::IntoSpaceAndHtmlAttributesOrEmpty
                for $($wrapper)*::<V>
            {
                type SpaceAndHtmlAttributesOrEmpty = ::async_str_iter::empty::Empty;

                fn into_space_and_html_attributes_or_empty(self) -> Self::SpaceAndHtmlAttributesOrEmpty {
                    ::async_str_iter::empty::Empty
                }
            }
        };
    }

    pub use __Ref_csr as csr;
    pub use __Ref_ssr as ssr;
}

macro_rules! impl_bounds {
    (
        $($wrapper_path_start:ident)? $(:: $wrapper_path:ident)* (
            prop_marker($($prop_marker:tt)+),
            $($(#$bounds_attrs:tt)+)?
            bounds as $($mod_path_start:ident)?
                $(:: $mod_path:ident)*
                $($(::)? <$($ty:ty),* $(,)?>)?,
            element as $csr_element_ty:ident,
            attr_name = $attr_name:expr
            $(, $name:ident $fields:tt)*
            $(,)?
        )
    ) => {
        $crate::impl_bounds! {@impl { $($mod_path_start)? $(:: $mod_path)* } {
            wrapper! { $($wrapper_path_start)? $(:: $wrapper_path)* }
            prop_marker! { $($prop_marker)+ }
            $(bounds_attrs! { $(#$bounds_attrs)+ })?
            bounds!  { $($mod_path_start)? $(:: $mod_path)* }
            bounds_tps! { $($($ty,)*)? }
            csr_element_ty! { $csr_element_ty }
            attr_name! { attr_name = $attr_name }
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

macro_rules! default_impl_csr {
    (
        meta! {
            wrapper! {$($wrapper:tt)*}
            prop_marker! {$($prop_marker:tt)*}
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
                crate::intrinsic::AttributeState<
                    ::core::marker::PhantomData<$($prop_marker)*>,
                    $($bounds)*::$csr::State![{$($bounds)*}[$($bounds_tp),*][V]],
                >;

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
                }, &mut state.1)
            }
        }
    };
}

macro_rules! default_impl_csr_without_attr_name {
    (
        meta! {
            wrapper! {$($wrapper:tt)*}
            prop_marker! {$($prop_marker:tt)*}
            bounds!  {$($bounds:tt)*}
            bounds_tps!  {$($bounds_tp:ty,)*}
            csr_element_ty! { $csr_element_ty:ident }
            $(attr_name! { $attr_name_ident:ident = $attr_name:expr })?
        }
        $csr:ident ! $csr_body:tt
    ) => {
        crate::impl_bounds::default_impl_csr! {
            meta! {
                wrapper! {$($wrapper)*}
                prop_marker! {$($prop_marker)*}
                bounds!  {$($bounds)*}
                bounds_tps!  {$($bounds_tp,)*}
                csr_element_ty! { $csr_element_ty }
            }
            $csr ! $csr_body
        }
    };
}

macro_rules! default_impl_ssr {
    (
        meta! {
            wrapper! {$($wrapper:tt)*}
            prop_marker! {$($prop_marker:tt)*}
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

macro_rules! DefaultCsrState {
    ({$($mod_path:tt)*}[$($($t0:tt)+)?][$($t1:tt)*]) => {
        $($mod_path)* ::csr::State::<$($($t0)*,)? $($t1)*>
    };
}

macro_rules! DefaultSsrHaevoe {
    ({$($mod_path:tt)*}[$($($t0:tt)+)?][$($t1:tt)*]) => {
        $($mod_path)* ::ssr::Haevoe::<$($($t0)*,)? $($t1)*>
    };
}

pub(crate) use {default_impl_csr, default_impl_ssr, impl_bounds};

#[allow(non_snake_case)]
pub(crate) mod DomTokens {
    pub(crate) use frender_dom::dom_tokens::DomTokens as Bounds;

    macro_rules! __csr_DomTokens {
        (
            meta! {
                wrapper! {$($wrapper:tt)*}
                prop_marker! {$($prop_marker:tt)*}
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
                    crate::intrinsic::AttributeState<
                        ::core::marker::PhantomData<$($prop_marker)*>,
                        $($bounds)*::$csr::State![{$($bounds)*}[$($bounds_tp),*][V]],
                    >;

                fn update_node_non_reactive<Renderer: $crate::RenderHtml + ?::core::marker::Sized>(
                    Self(this): Self,
                    renderer: &mut Renderer,
                    element: &mut ET::NodeOfBehaviorType<Renderer>,
                    crate::intrinsic::AttributeState(
                        ::core::marker::PhantomData,
                        state,
                    ): &mut Self::State<Renderer>,
                ) {
                    #[allow(unused_imports)]
                    use $crate::html::behaviors_prelude::$csr_element_ty::*;

                    let element = <<ET as $crate::html::behavior_type_traits::$csr_element_ty>::$csr_element_ty<Renderer> as frender_common::convert::FromMut<_>>::from_mut(element);

                    let input = $($bounds)*::$csr::Input {
                        this,
                        element,
                        renderer,
                        // $($attr_name_ident: $attr_name,)?
                        $($csr_fields)*
                    };

                    let mut dom_token_list = (input.get_mut_dom_token_list)(input.element, input.renderer);
                    V::update_with_state(input.this, &mut dom_token_list, state)
                }
            }
        };
    }

    pub(crate) use super::default_impl_ssr as ssr;
    pub(crate) use __csr_DomTokens as csr;

    pub(crate) mod csr {
        use frender_dom::dom_tokens::DomTokens;

        pub(crate) use DefaultCsrState as State;

        pub(crate) type State<V> = <V as DomTokens>::UpdateWithState;

        pub(crate) struct Input<'a, V, E: ?Sized, RR: ?Sized, F> {
            pub(crate) this: V,
            pub(crate) element: &'a mut E,
            pub(crate) renderer: &'a mut RR,
            pub(crate) get_mut_dom_token_list: F,
        }
    }

    pub(crate) mod ssr {
        use frender_dom::dom_tokens::DomTokens;

        pub(crate) use DefaultSsrHaevoe as Haevoe;

        pub(crate) type Haevoe<V> = frender_ssr::html::attr_value::AttrEqValue<<V as DomTokens>::DomTokensIntoAsyncStrIter>;

        pub(crate) fn maybe_into_haevoe<V: DomTokens>(this: V) -> Option<Haevoe<V>> {
            Some(Haevoe::<V>::new(V::dom_tokens_into_async_str_iter(this)))
        }
    }
}

mod updater {
    use std::marker::PhantomData;

    use frender_attr_value::csr::ValueKind;

    pub(super) struct UpdaterOfKind<'a, VK: ?Sized, E: ?Sized, RR: ?Sized, U, R> {
        pub(super) _kind: PhantomData<VK>,
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
        > frender_attr_value::csr::UpdateAttrValue for UpdaterOfKind<'a, VT, E, RR, U, R>
    {
        type Kind = VT;
        fn set(mut self, value: VT::Value<'_>) {
            (self.update)(&mut self.element, &mut self.renderer, self.attr_name, value)
        }

        fn remove(mut self) {
            (self.remove)(&mut self.element, &mut self.renderer, self.attr_name)
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod AttrValue {
    pub(crate) use frender_attr_value::AttrValue as Bounds;

    pub(crate) use default_impl_csr as csr;
    pub(crate) use default_impl_ssr as ssr;

    pub(crate) mod csr {
        use frender_attr_value::csr::{CsrAttrValue, ValueKind};

        pub(crate) use DefaultCsrState as State;

        use super::super::updater;

        pub(crate) struct Input<'a, V, E: ?Sized, RR: ?Sized, U, R> {
            pub(crate) this: V,
            pub(crate) element: &'a mut E,
            pub(crate) renderer: &'a mut RR,
            pub(crate) attr_name: &'static str,
            pub(crate) update: U,
            pub(crate) remove: R,
        }

        impl<'a, V, E: ?Sized, RR: ?Sized, U, R> Input<'a, V, E, RR, U, R> {
            fn into_value_and_updater<VK: ?Sized>(self) -> (V, updater::UpdaterOfKind<'a, VK, E, RR, U, R>) {
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
                    updater::UpdaterOfKind {
                        _kind: std::marker::PhantomData,
                        element,
                        renderer,
                        attr_name,
                        update,
                        remove,
                    },
                )
            }
        }

        // TODO: redesign state for attributes
        pub(crate) type State<VT, V> = Option<<V as CsrAttrValue<VT>>::State>;

        pub(crate) fn update_with_state<
            //
            VT: ?Sized + ValueKind,
            V: CsrAttrValue<VT>,
            E,
            RR: ?Sized,
            U: FnOnce(&mut E, &mut RR, &'static str, VT::Value<'_>),
            R: FnOnce(&mut E, &mut RR, &'static str),
        >(
            input: Input<V, E, RR, U, R>,
            state: &mut State<VT, V>,
        ) {
            let (this, updater) = input.into_value_and_updater();
            V::update_attribute_value_with_option_state(this, updater, state)
        }
    }

    pub(crate) mod ssr {
        use frender_attr_value::ssr::SsrAttrValue;

        pub(crate) use DefaultSsrHaevoe as Haevoe;

        pub(crate) type Haevoe<VT, V> = <V as SsrAttrValue<VT>>::HtmlAttributeValue;

        pub(crate) fn maybe_into_haevoe<VT: ?Sized, V: SsrAttrValue<VT>>(this: V) -> Option<Haevoe<VT, V>> {
            V::maybe_into_html_attribute_value(this)
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod MaybeHandleEvent {
    macro_rules! __impl_csr_MaybeHandleEvent {
        (
            meta! {
                wrapper! {$($wrapper:tt)*}
                prop_marker! {$($prop_marker:tt)*}
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
                    $($bounds_tp)*::EventListenerOf<
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

    pub(crate) use __impl_csr_MaybeHandleEvent as csr;

    macro_rules! __impl_ssr_MaybeHandleEvent {
        (
            meta! {
                wrapper! {$($wrapper:tt)*}
                prop_marker! {$($prop_marker:tt)*}
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

    pub(crate) use __impl_ssr_MaybeHandleEvent as ssr;
}

#[allow(non_snake_case)]
pub(crate) mod SetRef {
    macro_rules! __Ref_csr {
        (
            meta! {
                wrapper! {$($wrapper:tt)*}
                prop_marker! {$($prop_marker:tt)*}
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

    macro_rules! __Ref_ssr {
        (
            meta! {
                wrapper! {$($wrapper:tt)*}
                prop_marker! {$($prop_marker:tt)*}
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

    pub(crate) use __Ref_csr as csr;
    pub(crate) use __Ref_ssr as ssr;
}

#[allow(non_snake_case)]
pub(crate) mod Style {
    pub(crate) use frender_style::Style as Bounds;

    pub(crate) use default_impl_csr_without_attr_name as csr;
    pub(crate) use default_impl_ssr as ssr;

    pub(crate) mod csr {
        use frender_style::csr::CsrStyle;

        pub(crate) struct Input<'a, V, E: ?Sized, R: ?Sized> {
            pub(crate) this: V,
            pub(crate) element: &'a mut E,
            pub(crate) renderer: &'a mut R,
        }

        pub(crate) use DefaultCsrState as State;

        pub(crate) type State<V> = <V as CsrStyle>::UpdateWithState;

        pub(crate) fn update_with_state<
            //
            V: CsrStyle,
            E: frender_dom::behaviors::ElementWithStyle<RR>,
            RR: ?Sized,
        >(
            Input { this, element, renderer }: Input<V, E, RR>,
            state: &mut State<V>,
        ) {
            V::update_with_state(this, state, &mut element.style(renderer))
        }
    }

    pub(crate) mod ssr {
        use frender_style::ssr::{SsrDeclarationList, SsrStyle};

        pub(crate) use DefaultSsrHaevoe as Haevoe;

        pub(crate) type Haevoe<V> = frender_ssr::html::attr_value::AttrEqValue<<<V as SsrStyle>::IntoSsrDeclarationList as SsrDeclarationList>::IntoDeclarationList>;

        pub(crate) fn maybe_into_haevoe<V: SsrStyle>(this: V) -> Option<Haevoe<V>> {
            Some(Haevoe::<V>::new(SsrDeclarationList::into_declaration_list(V::into_ssr_declaration_list(this))))
        }
    }
}

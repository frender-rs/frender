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
        #[cfg(todo)]
        impl<
            V: $($bounds)*::Bounds::<$($bounds_tp,)*>,
            ET: $crate::html::behavior_type_traits::$csr_element_ty,
        >
            $crate::update_element::UnpinnedRenderWithBehavior<
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
                element: &mut ET::OfBehaviorType<Renderer>,
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

        #[cfg(todo)]
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
                element: &mut ET::OfBehaviorType<Renderer>,
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
            #[cfg(todo)]
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
                    element: &mut ET::OfBehaviorType<Renderer>,
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
            > $crate::dom::ssr::IntoSpaceAndHtmlAttributesOrEmpty
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

pub(crate) use crate::dom_tokens::impl_bounds as DomTokens;
pub(crate) use crate::style::impl_bounds as Style;

#[cfg(feature = "ssr")]
pub(crate) mod ssr;

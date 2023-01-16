use bg::{Maybe, MaybeBorrow, Unspecified};
use frender_dom::props::MaybeRenderingStr;

pub trait IntrinsicComponent {
    const INTRINSIC_TAG: &'static str;
}

pub struct FieldData<'a, Data, State, Element> {
    pub data: Data,
    pub state: State,
    pub element: &'a Element,
    pub dom_element: &'a ::web_sys::Element,
    pub children_ctx: &'a mut ::frender_dom::Dom,
}

macro_rules! expand_a_or_b {
    ($({$($prefix:tt)*})?[][$($b:tt)*]$({$($suffix:tt)*})?) => {
        $($($prefix)*)?
        $($b)*
        $($($suffix)*)?
    };
    ($({$($prefix:tt)*})?[$($a:tt)+][$($b:tt)*]$({$($suffix:tt)*})?) => {
        $($($prefix)*)?
        $($a)+
        $($($suffix)*)?
    };
}

macro_rules! __impl_mod_overwrite {
    (+ $fields:tt) => {
        __impl_mod_overwrite! {
            @ $fields $fields
        }
    };
    (- $field:ident [$($fields:ident)*]) => {
        dyn super::Types<
            $(
                $fields = $field![$fields TypeDefs Value],
            )*
        >
    };
    (@ [$($field:ident)*] $fields:tt) => {
        $(
            macro_rules! $field {
                ($field $base:ident $new_ty:ident) => {
                    $new_ty
                };
                ($other_field:ident $base:ident $new_ty:ident) => {
                    <$base as super::Types>::$other_field
                };
            }

            pub type $field<TypeDefs, Value> = __impl_mod_overwrite![
                - $field $fields
            ];
        )*
    };
}

macro_rules! impl_builder_fn_body {
    (
        $name:ident
        $this:ident
        $field:ident = $field_value:expr;
        {$($fields:ident)*}
    ) => {
        let __tmp_value = $field_value;

        let $name::Data {
            _phantom_marker,
            $($fields),*
        } = $this.0;

        let _ = $field;

        let $field = __tmp_value;

        $name::Building(
            $name::Data {
                _phantom_marker,
                $($fields),*
            }
        )
    };
}

macro_rules! impl_builder_fns {
    (
        $name:ident {$(
            $([$($marker_bounds:tt)+])?
            {$(
                $field:ident $([$($bounds:tt)+])?
            ),* $(,)?}
        )*}
        $fields:tt
    ) => {$(
        impl<TypeDefs: $name::Types, Marker $(: $($marker_bounds)+)?>
        $name::Building<TypeDefs, Marker> {$(
            pub fn $field<V $(: $($bounds)+)?>(
                self,
                $field: V,
            ) -> $name::Building<
                $name::overwrite::$field<TypeDefs, V>,
                Marker,
            > {
                impl_builder_fn_body! {
                    $name self
                    $field = $field;
                    $fields
                }
            }
        )*}
    )*};
}

macro_rules! def_intrinsic_component_props {
    (
        $vis:vis struct $name:ident = $default_marker:ty {$(
            $([$($marker_bounds:tt)+])?
            {$(
                $field:ident
                $(
                    ? &str $((
                        $($prop_name:literal)? $($element_method:ident)?
                    ))?
                )?
                $(
                    @ $marker_method:ident ($($marker_defs:tt)*)
                )?
                $(
                    $([$($bounds:tt)+])?
                    : $initial_ty:ty
                    = $initial_value:expr
                    => $defs:tt
                )?
            ),* $(,)?}
        )*}
    ) => {
        __impl_def_intrinsic_component_props! {
            $vis struct $name = $default_marker {$(
                $([$($marker_bounds)+])?
                {$(
                    $field
                    $(
                        [::frender_dom::props::MaybeRenderingStr]: () = () => {
                            dom {
                                state
                                    < <TypeDefs::$field as ::frender_dom::props::MaybeRenderingStr>::State >
                                impl { data, dom_element, state, .. } {
                                    const ATTR_NAME: &::core::primitive::str = expand_a_or_b!(
                                        [$($($prop_name)?)?]
                                        [::core::stringify!($field)]
                                    );
                                    ::frender_dom::props::MaybeRenderingStr::maybe_update_rendering_str_with_cache(
                                        data,
                                        state,
                                        |v| expand_a_or_b!(
                                            [$($(dom_element.$element_method(v))?)?]
                                            [dom_element.set_attribute(ATTR_NAME, v).unwrap()]
                                        ),
                                        || dom_element.remove_attribute(ATTR_NAME).unwrap()
                                    )
                                }
                            }
                        }
                    )?

                    $(
                        @ $marker_method:ident ($($marker_defs:tt)*)
                    )?

                    $(
                        $([$($bounds)+])?
                        : $initial_ty
                        = $initial_value => $defs
                    )?
                ),*}
            )*}
        }
    };
}

macro_rules! __impl_def_intrinsic_component_props {
    (
        $vis:vis struct $name:ident = $default_marker:ty {$(
            $([$($marker_bounds:tt)+])?
            {$(
                $field:ident $([$($bounds:tt)+])?
                : $initial_ty:ty
                = $initial_value:expr => {
                    dom {
                        $(bounds [$($dom_bounds:tt)+])?
                        $(
                            state $($pin:ident)?
                            <$dom_state_ty:ty>
                            $(:[$($dom_state_bounds:tt)+]=($dom_state_init:expr))?
                        )?
                        impl $dom_pat:tt $dom_impl:tt
                    }
                }
            ),* $(,)?}
        )*}
    ) => {
        #[allow(non_snake_case)]
        $vis mod $name {
            pub mod overwrite {
                #![allow(non_camel_case_types)]

                __impl_mod_overwrite! { +[$($($field)*)*] }
            }

            mod trait_types {
                use super::super::*;
                #[allow(non_camel_case_types)]
                pub trait Types {$($(
                    type $field $(: $($bounds)+)? ;
                )*)*}
            }

            pub use trait_types::Types;
            pub use trait_types::Types as ValidTypes;

            pub mod data_struct {
                #[non_exhaustive]
                pub struct $name<TypeDefs: super::Types + ?::core::marker::Sized, Marker = $default_marker> {
                    pub(in super::super) _phantom_marker: ::core::marker::PhantomData<Marker>,
                    $($(
                        pub $field : TypeDefs::$field,
                    )*)*
                }
            }

            pub use data_struct::$name as Data;

            pub struct Building<TypeDefs: ?::core::marker::Sized + Types, Marker = $default_marker>(
                pub Data<TypeDefs, Marker>,
            );

            mod types_initial {
                use super::super::*;
                pub type TypesInitial = dyn super::Types<$($(
                    $field = $initial_ty,
                )*)*>;
            }
            pub use types_initial::TypesInitial;

            pub type DataInitial = Data<TypesInitial>;

            pub mod render_state {
                #[allow(non_camel_case_types)]
                pub trait RenderStateTypes {$($(
                    expand_a_or_b! {
                        { type $field : }
                        [$($($($dom_state_bounds)+)?)?]
                        [::core::default::Default]
                        {;}
                    }
                )*)*}
                ::pin_project_lite::pin_project! {
                    #[project = RenderStateProj]
                    pub struct RenderState<TypeDefs: RenderStateTypes, Element>
                    where TypeDefs: ?::core::marker::Sized {
                        pub node_and_mounted: Option<(Element, bool)>,
                        $($(
                            $($(#[$pin])?)?
                            pub $field: TypeDefs::$field,
                        )*)*
                    }
                }

                impl<
                    TypeDefs: ?::core::marker::Sized + RenderStateTypes,
                    Element,
                > RenderState<TypeDefs, Element> {
                    #[inline]
                    pub(crate) fn pin_project(self: ::core::pin::Pin<&mut Self>) -> RenderStateProj<'_, TypeDefs, Element> {
                        self.project()
                    }
                }

                impl<
                    TypeDefs: ?::core::marker::Sized + RenderStateTypes,
                    Element: ::core::convert::AsRef<::web_sys::Element>,
                > ::frender_core::RenderState for RenderState<TypeDefs, Element> {
                    fn new_uninitialized() -> Self {
                        Self {
                            node_and_mounted: ::core::option::Option::None,
                            $($(
                                $field: expand_a_or_b![
                                    [$($($dom_state_init)?)?]
                                    [::core::default::Default::default()]
                                ],
                            )*)*
                        }
                    }

                    fn unmount(self: ::core::pin::Pin<&mut Self>) {
                        let mounted = self.node_and_mounted.as_ref().map_or(false, |v| v.1);
                        if !mounted {
                            return;
                        }

                        let this = self.project();
                        if let Some((node, mounted)) = this.node_and_mounted {
                            node.as_ref().remove();
                            *mounted = false;
                        }
                    }
                }
            }
        }

        impl_builder_fns! {
            $name {$(
                $([$($marker_bounds)+])?
                {$(
                    $field $([$($bounds)+])?
                ),*}
            )*}
            {$($($field)*)*}
        }

        impl<
            TypeDefs: ?::core::marker::Sized + $name::Types,
            Marker: $crate::props::element_types::ElementTypeMarker
        > ::frender_core::UpdateRenderState<::frender_dom::Dom> for $name::Data<TypeDefs, Marker>
        where
            $($(
                $(TypeDefs::$field : $($dom_bounds)+)?
            )*)*
        {
            type State = $name::render_state::RenderState<
                dyn $name::render_state::RenderStateTypes<$($(
                    $field = expand_a_or_b![[$($dom_state_ty)?][()]],
                )*)*>,
                Marker::Element,
            >;

            fn update_render_state(self, ctx: &mut ::frender_dom::Dom, state: ::core::pin::Pin<&mut Self::State>) {
                let state = state.pin_project();
                Marker::mount_element_and_update(
                    state.node_and_mounted,
                    ctx,
                    |element, children_ctx| {
                        let dom_element: &::web_sys::Element = element.as_ref();



                        $($(
                            match ($crate::props::FieldData {
                                data: self.$field,
                                state: state.$field,
                                element,
                                dom_element,
                                children_ctx: &mut *children_ctx,
                            }) {
                                $crate::props::FieldData $dom_pat => $dom_impl
                            }
                        )*)*
                    }
                );
            }
        }

        #[allow(non_snake_case)]
        $vis fn $name () -> $name::Building<$name::TypesInitial> {
            $name::Building(
                $name::Data {
                    _phantom_marker: ::core::marker::PhantomData,
                    $($(
                        $field : $initial_value,
                    )*)*
                }
            )
        }
    };
}

def_intrinsic_component_props! {
    pub struct HtmlCommonSharedProps = crate::props::element_types::HtmlElement {
        {
            children: () = () => {
                dom {
                    bounds[::frender_core::UpdateRenderState<::frender_dom::Dom>]
                    state pin
                        < <TypeDefs::children as frender_core::UpdateRenderState<frender_dom::Dom>>::State >
                        :[::frender_core::RenderState]=(::frender_core::RenderState::new_uninitialized())
                    impl { data, state, children_ctx, .. } {
                        data.update_render_state(children_ctx, state)
                    }
                }
            },
            class ? &str,
            id ? &str (set_id),
            part ? &str,

            default_checked[Maybe<bool>]: Unspecified<bool> = Unspecified => {
                dom {
                    impl { data, element, .. } {
                        Marker::update_property_default_checked(element, data)
                    }
                }
            },
        }
    }
}

#[cfg(aaa)]
bg::builder! {
    #![inheritable]
    pub struct HtmlCommonSharedProps {
        children[impl Sized],
        draggable[? bool],
        hidden[? bool],
        id[borrow? str],
        lang[borrow? str],
        placeholder[borrow? str],
        style[impl crate::props::UpdateStyleProperty],
        tab_index[? i32],
        title[borrow? str],

        // TODO: ref_element
        // ref_el[TWriteRef: 'static + react::WriteRef<TElement> + react::SafeIntoJsRuntime][? TWriteRef] { safe_into_js_runtime? },

        // Standard HTML Attributes
        access_key[borrow? str],
        content_editable[impl crate::props::MaybeInherit<bool>],
        context_menu[borrow? str],
        dir[borrow? str],
        slot[borrow? str],
        spell_check[? bool],
        translate[borrow? str], // TODO: ser: yes | no

        // Unknown
        radio_group[borrow? str], // <command>, <menuitem>

        // WAI-ARIA
        role[? crate::aria::Role],

        // RDFa Attributes
        about[borrow? str],
        datatype[borrow? str],
        inlist[impl Sized],
        prefix[borrow? str],
        property[borrow? str],
        resource[borrow? str],
        type_of[borrow? str],
        vocab[borrow? str],

        // Non-standard Attributes
        auto_capitalize[borrow? str],
        auto_correct[borrow? str],
        auto_save[borrow? str],
        color[borrow? str],
        item_prop[borrow? str],
        item_scope[? bool],
        item_type[borrow? str],
        item_id[borrow? str],
        item_ref[borrow? str],
        results[? i32],
        security[borrow? str],
        unselectable[borrow? str], // TODO: ser: 'on' | 'off' | undefined;

        // Living Standard
        input_mode[? crate::HtmlInputMode],
        is[borrow? str],

        __html_element_event_listeners[impl Sized]: crate::props::HtmlElementEventListeners::DataInitial = crate::props::HtmlElementEventListeners(),
    }
}

pub trait UpdateHtmlElementEventListeners {
    type State;
    fn update_dom_event_listeners(self, element: &web_sys::HtmlElement, state: &mut Self::State);
}

macro_rules! __impl_use_custom_element {
    (ident {}) => {
        react::Element
    };
    (ident {$el:ty}) => {
        $el
    };
    (from_element {} $e:expr) => {
        $e
    };
    (from_element {$el:ty} $e:expr) => {
        <$el>::private_from_element($e)
    };
}

macro_rules! __impl_ignore_first {
    ({$($ignore:tt)*} $($t:tt)*) => {
        $($t)*
    };
}

macro_rules! __impl_def_component_struct {
    ($comp:ident) => {
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $comp;
    };
    ([$($impl_generics:tt)+] $comp:ident [$($type_params:ident),+ $(,)?]) => {
        #[derive(Clone, Copy, Default)]
        pub struct $comp<$($impl_generics)+> {
            _phantom: PhantomData<($($type_params),+ ,)>,
        };
        impl<$($impl_generics)+> std::fmt::Debug for $comp<$($type_params),+> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, concat!(stringify!($comp), "<",
                        $($crate::macros::__impl_ignore_first({$type_params} "{}, ") ,)+
                    ">"), $(std::any::type_name::<$type_params>())+ )
            }
        }
    };
}

macro_rules! def_intrinsic_component {
    (
        // "tag"
        $tag:tt
        // MyComponent
        $([$($impl_generics:tt)+])? $comp:ident $([$($type_params:tt)+])? ( $props:ty )
        // => MyCustomElement
        $(=> $el:ty)?
        $({
            $([ $($props_generics:tt)+ ])? $props_struct_name:ident $([ $($props_type_params:tt)+ ])?
            : $props_trait_name:ident
            $($def_props:tt)*
        })?
    ) => {
        pub mod prelude {
            $(
                pub use super:: $props_trait_name;
                pub use super:: $props_struct_name as ComponentProps;
            )?
            pub use crate::HtmlBasePropsBuilder;
            pub use super:: $comp as Component;
        }

        $(
            $crate::macros::def_props_trait! {
                $([ $($props_generics)+ ])? $props_struct_name $([ $($props_type_params)+ ])?
                : $props_trait_name
                $($def_props)*
            }
        )?

        $crate::macros::__impl_def_component_struct! {
            $([$($impl_generics)+])? $comp $([$($type_params)+])?
        }

        impl $(<$($impl_generics)+>)? $crate::IntrinsicComponent for $comp $(<$($type_params)+>)? {
            const INTRINSIC_TAG: &'static str = $tag;
        }

        impl $(<$($impl_generics)+>)? react::UseRenderStatic for $comp $(<$($type_params)+>)? {
            type RenderArg = $props;
            type RenderOutput = $crate::macros::__impl_use_custom_element! {ident {$($el)?}};

            #[inline]
            fn use_render(props: &Self::RenderArg) -> Self::RenderOutput {
                use $crate::IntrinsicComponent;
                use react::any_js_props::AnyJsProps;
                let AnyJsProps {
                    js_props_without_children,
                    children,
                } = props.as_ref().props.clone();
                $crate::macros::__impl_use_custom_element!(from_element {$($el)?} react::JsBridgeElement {
                    js_component_type: react::JsComponentType::StaticIntrinsic(Self::INTRINSIC_TAG),
                    js_props_without_children,
                    children,
                    key: None,
                }.into())
            }
        }

        impl $(<$($impl_generics)+>)? react::ComponentStatic for $comp $(<$($type_params)+>)? {
            type Props = $props;
            type Element = $crate::macros::__impl_use_custom_element! {ident {$($el)?}};

            #[inline]
            fn create_element(props: Self::Props, key: Option<react::Key>) -> Self::Element {
                // use $crate::IntrinsicComponent;
                use react::UseRenderStatic;
                use react::IntoOptionalElement;
                $crate::macros::__impl_use_custom_element!(from_element {$($el)?} react::Element::bridge_use_render(
                    move || Self::use_render(&props).into_optional_element(),
                    key,
                    None,
                    None,
                ))
            }
        }
    };
}

pub(crate) use __impl_def_component_struct;
pub(crate) use __impl_use_custom_element;
pub(crate) use def_intrinsic_component;

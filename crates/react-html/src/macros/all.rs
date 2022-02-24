macro_rules! intrinsic_component_tag {
    ($tag:ident) => {
        pub mod $tag;
    };
    ($tag:ident : $comp:ident) => {
        pub mod $tag {
            $crate::macros::def_intrinsic_component! {
                (stringify!($tag))
                $comp ( $crate::HtmlCommonSharedProps<web_sys::HtmlElement> )
            }
        }
    };
    ($tag:ident : { $comp:ident => $el:ty }) => {
        pub mod $tag {
            $crate::macros::def_intrinsic_component! {
                (stringify!($tag))
                $comp ( $crate::HtmlCommonSharedProps<$el> )
            }
        }
    };
}

macro_rules! all_intrinsic_component_tags {
    ($($tag:ident $(: $option:tt)?),+ $(,)?) => {
        $(
            $crate::macros::intrinsic_component_tag! {
                $tag
                $(: $option)?
            }
        )+
    };
}

pub(crate) use all_intrinsic_component_tags;
pub(crate) use intrinsic_component_tag;

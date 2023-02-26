mod debug;
pub use debug::*;

pub use frender_core::{Keyed, RenderState, StaticText, UpdateRenderState};
pub use frender_macros::{component, def_props};

#[cfg(feature = "dom")]
pub use frender_hook_element::frender_dom::Dom;

pub use frender_html::props::{events, UpdateDomEventListener};
pub use frender_html_components::intrinsic_components;

pub use frender_hook_element as hook_element;

pub mod prelude {
    pub use bg;
    pub use bg::{Maybe as _, MaybeBorrow as _};

    pub use crate::{render, rsx};

    pub use frender_core::{Keyed, StaticText};
    pub use frender_html_components::intrinsic_components;
    pub use frender_macros::component;
}

#[macro_export]
macro_rules! rsx_xml {
    (< $name:ident $next:tt $($t:tt)*) => {
        $crate::__impl_rsx_xml_ident_next! {
            $next
            $name
            $next
            $($t)*
        }
    };
    (< $($t:tt)*) => {
        $crate::__private::rsx_xml_with_full_path! {
            < $($t)*
        }
    }
}

#[macro_export]
macro_rules! render {
    ($ctx:expr => $($t:tt)*) => {
        $crate::__private::frender_hook_element::ContextAndState::render($ctx, $crate::rsx!($($t)*))
    };
}

#[macro_export]
macro_rules! rsx {
    (< $($t:tt)*) => {
        $crate::rsx_xml! {
            < $($t)*
        }
    };
    ($lit:literal) => { $lit };
    ({ $($braced:tt)* }) => { $($braced)* };
    (
        $($t:tt)*
    ) => {
        $crate::rsx_build_element! { $($t)* }
    };
}

#[doc(hidden)]
pub mod __private {
    pub use frender_hook_element;

    pub use frender_macros::rsx_xml_with_full_path;

    pub use frender_macros::__impl_auto_prepend_intrinsic_components;
}

#[macro_export]
macro_rules! __impl_rsx_xml_ident_next {
    (
        / $($t:tt)*
    ) => {
        $crate::__private::__impl_auto_prepend_intrinsic_components! {
            [$crate::__private::rsx_xml_with_full_path]
            [<]
            $($t)*
        }
    };
    (
        > $($t:tt)*
    ) => {
        $crate::__private::__impl_auto_prepend_intrinsic_components! {
            [$crate::__private::rsx_xml_with_full_path]
            [<]
            $($t)*
        }
    };
    (
        $_prop:ident $($t:tt)*
    ) => {
        $crate::__private::__impl_auto_prepend_intrinsic_components! {
            [$crate::__private::rsx_xml_with_full_path]
            [<]
            $($t)*
        }
    };
    (
        $($t:tt)*
    ) => {
        $crate::__private::rsx_xml_with_full_path! {
            < $($t)*
        }
    };
}

#[macro_export]
macro_rules! rsx_build_element {
    (
        $maybe_intrinsic:ident
        ($($inner:tt)*)
        $($t:tt)*
    ) => {
        $crate::__private::__impl_auto_prepend_intrinsic_components! {
            [$crate::prelude::bg::finish_builder_with]
            [[build_element]]
            $maybe_intrinsic
            ($($inner)*)
            $($t)*
        }
    };
    (
        $($t:tt)*
    ) => {
        $crate::prelude::bg::finish_builder_with!(
            [build_element]
            $($t)*
        )
    };
}

pub use frender_dom as dom;

pub mod component;

pub use frender_html_common::{AsyncWritableAttrs, DomTokens, IntoAsyncWritableAttrs};

pub mod convert;
pub mod renderer;

mod element;
mod update_element;
pub use element::*;
pub use update_element::*;

pub mod elements;
pub mod pin_mut_maybe_uninit;

mod render_element;
pub use render_element::RenderElement;

pub use frender_html_common::dom_token::DomTokenList;

// pub mod data_transfer;

pub use frender_common::expand;

mod shims;

pub mod html;
pub use html::RenderHtml;

pub mod impl_bounds;

pub mod __private {
    pub use frender_common::write::attrs::IntoAsyncWritableAttrs;
}

pub mod html_imports {
    pub use crate::{
        impl_bounds::{Css, DomTokens, MaybeContentEditable},
        renderer::RenderTextFrom,
    };
}

#[macro_export]
macro_rules! expand_html_traits {
    ($(
        #[$macro_name:ident]
        $item_vis:vis $item_type:ident $item_name:ident $item_body_or_semi:tt
    )*) => {
        use $crate::html_imports::*;
        use $crate::html::*;

        $crate::html::__impl_expand_html_traits! {{
            wrap {}
            append($(
                $macro_name ( $item_vis $item_type $item_name $item_body_or_semi )
            )*)
            wrap {}
            prepend { $crate::define_item_and_traverse_traits! }
        }}
    };
    ($macro_name:ident $bang:tt) => {
        $crate::html::__impl_expand_html_traits! {{
            for_each {
                wrap {}
                prepend(
                    $crate::$macro_name $bang
                )
            }
        }}
    };
    ($commands:tt) => {
        $crate::html::__impl_expand_html_traits! { $commands }
    };
}

#[cfg(feature = "html_builders_not_expanded")]
mod html_builders {
    use super::html::*;
    crate::expand_html_traits!(
        #[props_builders]
        mod props_builders;
    );
}

#[cfg(not(feature = "html_builders_not_expanded"))]
mod html_builders;

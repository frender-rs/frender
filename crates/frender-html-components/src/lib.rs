pub mod html;

#[cfg(feature = "html_macro_not_expand")]
mod html_builders {
    use super::html::*;
    frender_html::expand_html_traits!(
        #[props_builders]
        mod props_builders;
    );
}

#[cfg(not(feature = "html_macro_not_expand"))]
mod html_builders;

pub use html::components as html_components;
pub use html::components as intrinsic_components;

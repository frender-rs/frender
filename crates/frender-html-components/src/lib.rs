mod common;

pub mod html {

    use frender_html::html::*;
    // TODO: move to a common mod
    use frender_html::{
        impl_bounds::{Css, DomTokens, MaybeContentEditable},
        renderer::RenderTextFrom,
    };
    pub mod props {
        frender_html::expand_html_traits! {props!}
    }
    pub mod components {
        frender_html::expand_html_traits! {components!}
    }
}

pub use html::components as html_components;
pub use html::components as intrinsic_components;

mod common;

pub mod html {

    frender_html::expand_html_traits!(
        #[props]
        pub mod props;
        #[components]
        pub mod components;
    );
}

pub use html::components as html_components;
pub use html::components as intrinsic_components;

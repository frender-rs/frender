mod common;

pub mod html {

    frender_html::expand_html_traits!(
        #[props_without_builders]
        pub mod props;

        #[props_builders]
        mod props_builders;

        #[components]
        pub mod components;
    );
}

pub use html::components as html_components;
pub use html::components as intrinsic_components;

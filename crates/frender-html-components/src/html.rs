frender_html::expand_html_traits!(
    #[props_without_builders]
    pub mod props;

    #[props_builders]
    mod props_builders;

    #[components]
    pub mod components;
);

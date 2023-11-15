mod common;

pub mod html {
    use frender_html::{
        html::*,
        // TODO: remove
        impl_bounds::{Css, DomTokens, MaybeContentEditable},
    };

    pub mod props {
        frender_html::expand_html_traits! {{
            for_each {
                wrap {}
                prepend( ::frender_html::props! )
            }
        }}
    }

    pub mod components {
        macro_rules! component {
            (
                extends($($extends:ident)*)
                $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
                $(special_inter_traits($($special_inter_traits:ident),* $(,)?))?
                vis($vis:vis)
                trait_name($trait_name:ident)
                $(define(
                    $(tags: ($($tags:ident),* $(,)?))?
                    $(,)?
                ))?
                $(verbatim_trait_items $verbatim_trait_items:tt)?
                $(impl_for_web $impl_for_web:tt)?
                fns $fns:tt
            ) => {
                ::frender_html::expand! {
                    while ($($($({$tags})*)?)?) {
                        prepend( $trait_name $vis )
                        wrap {}
                        prepend( ::frender_html::define_component! )
                    }
                }
            };
        }

        frender_html::expand_html_traits! {{
            for_each {
                wrap {}
                prepend( component! )
            }
        }}
    }
}

pub use html::components as html_components;

pub mod intrinsic_components {
    pub use super::html_components::*;
}

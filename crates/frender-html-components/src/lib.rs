mod common;

#[cfg(feature = "html_macro_not_expand")]
pub mod html;

// #[cfg(feature = "html_macro_not_expand")]
pub mod element_macros;

// #[cfg(not(feature = "html_macro_not_expand"))]
// pub mod html_expanded;
// #[cfg(not(feature = "html_macro_not_expand"))]
// pub use html_expanded as html;

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

// mod special_implementations;

mod imports {
    pub use frender_events;
    pub use frender_html;
    pub use frender_html_simple;
    pub use pin_project_lite::pin_project;

    pub use frender_html::impl_bounds;

    #[cfg(feature = "csr")]
    pub use frender_csr;

    #[cfg(feature = "ssr")]
    pub use frender_ssr;

    // #[cfg(feature = "html_macro_not_expand")]
    pub(crate) use super::element_macros::*;

    #[cfg(feature = "fully-typed")]
    pub(crate) use crate::ignore_first_ty::ignore_first_ty;
}

#[cfg(feature = "fully-typed")]
pub use html::fully_typed;
#[cfg(feature = "fully-typed")]
pub mod ignore_first_ty;

pub use html::components as html_components;

#[cfg(remove)]
pub mod html_components {
    pub use super::html::components::{
        //
        a,
        abbr,
        address,
        area,
        article,
        aside,
        audio,
        b,
        base,
        bdi,
        bdo,
        blockquote,
        body,
        br,
        button,
        canvas,
        caption,
        cite,
        code,
        col,
        colgroup,
        data,
        datalist,
        dd,
        del,
        details,
        dfn,
        dialog,
        div,
        dl,
        dt,
        em,
        embed,
        fieldset,
        figcaption,
        figure,
        footer,
        form,
        h1,
        h2,
        h3,
        h4,
        h5,
        h6,
        head,
        header,
        hgroup,
        hr,
        html,
        i,
        iframe,
        img,
        input,
        ins,
        kbd,
        label,
        legend,
        li,
        link,
        main,
        map,
        mark,
        menu,
        meta,
        meter,
        nav,
        noscript,
        object,
        ol,
        optgroup,
        option,
        output,
        p,
        picture,
        pre,
        progress,
        q,
        rp,
        rt,
        ruby,
        s,
        samp,
        script,
        section,
        select,
        slot,
        small,
        source,
        span,
        strong,
        style,
        sub,
        summary,
        sup,
        table,
        tbody,
        td,
        template,
        textarea,
        tfoot,
        th,
        thead,
        time,
        title,
        tr,
        track,
        u,
        ul,
        var,
        video,
        wbr,
    };
}

#[cfg(feature = "simply-typed")]
pub mod intrinsic_components {
    pub use super::html_components::*;
}

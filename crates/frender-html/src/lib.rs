#[cfg(feature = "html_macro_not_expand")]
mod element_macros;
#[cfg(feature = "html_macro_not_expand")]
pub use element_macros::*;

#[cfg(feature = "html_macro_not_expand")]
use frender_macros::def_intrinsic_component_props;

pub use frender_dom::props::{MaybeUpdateValue, MaybeUpdateValueByRef, MaybeUpdateValueWithState};

pub mod __private {
    pub use frender_core::RenderState;
    pub use pin_project_lite::pin_project;
}

mod data_types;
pub use data_types::*;

pub mod props;

#[cfg(feature = "html_macro_not_expand")]
pub mod html;

#[cfg(not(feature = "html_macro_not_expand"))]
mod html_expanded;
#[cfg(not(feature = "html_macro_not_expand"))]
pub use html_expanded::html;

mod utils;

pub mod html_components {
    pub use super::html::{
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

pub mod intrinsic_components {
    pub use super::html_components::*;
}

pub use crate::{
    csr::element::{
        HtmlRenderContext, PinnedRenderStateKind, PinnedRenderStateKindPollRender, PinnedStateOfKind, PinnedUiHandleOfKind, PinnedUnmountedUiHandleOfKind, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender,
        UnpinnedStateOfKind, UnpinnedUiHandleOfKind, UnpinnedUnmountedUiHandleOfKind,
    },
    html::RenderHtml,
};

pub use frender_reactive_value::RenderInitPinned;

pub use frender_dom::csr::{ProvideMutMounted, ProvideRenderContext};

pub mod kinds {
    pub use crate::csr::kinds::KindUnpinned;
}

pub mod render {
    pub use frender_dom::csr::render::{Render, RenderTextFrom};
}

pub mod render_from {
    pub use frender_dom::csr::render_from::*;
}

pub mod event_listener {
    pub use frender_dom::csr::{OnEvent, PinnedRegisterUpdate, RegisterUpdate};
}

pub mod events {
    pub use frender_dom::HasEventTypeName;
}

pub mod event_types {
    pub use frender_events::event_types::*;
}

pub mod behaviors {
    pub use frender_dom::csr::behaviors::{
        Element, ElementWithChildren, ElementWithClassList, ElementWithRelList, ElementWithStyle, HtmlElement, Node, NodeRenderSelf, NodeWithRenderContextAfterSelf, SetInnerHtmlFromStr, SetInnerTextFromStr,
    };
}

pub mod node_ref {
    pub use frender_dom::node_ref::traits;
}

#[cfg(feature = "web")]
pub mod web {
    pub use frender_dom::csr::web::{Cursor, CursorPlaceholder, Node, RenderContext, Renderer};
}

pub mod html {
    pub use crate::html::behaviors;
}

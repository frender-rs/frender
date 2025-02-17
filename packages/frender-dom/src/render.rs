pub use frender_csr_core::render::{RenderContext, RenderWithContext};

pub use self::text::{
    KnownValueForText, KnownValueKindForText, RenderTextFrom, RenderTextFromKnown, TextKind,
};

use crate::ui_handle::UiHandle;

mod text;

pub trait Render: RenderWithContext {
    fn log(&mut self, v: &str);

    type CursorPlaceholder: 'static
        + UiHandle<Self>
        + crate::behaviors::NodeRenderSelf<Self>
        + crate::behaviors::NodeWithRenderContextAfterSelf<Self>
        + crate::behaviors::Node<Self>;
}

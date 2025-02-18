pub use frender_csr_core::render::{RenderContext, RenderWithContext};

pub use self::text::{
    KnownValueForText, KnownValueKindForText, RenderTextFrom, RenderTextFromKnown, TextKind,
};

use crate::csr::{behaviors, ui_handle::UiHandle};

mod text;

pub trait Render: RenderWithContext {
    fn log(&mut self, v: &str);

    type CursorPlaceholder: 'static
        + UiHandle<Self>
        + behaviors::NodeRenderSelf<Self>
        + behaviors::NodeWithRenderContextAfterSelf<Self>
        + behaviors::Node<Self>;
}

pub use frender_csr::render::{RenderContext, RenderWithContext};

use crate::{string_element::StringElement, ui_handle::UiHandle};

pub trait RenderTextFrom<Text: UiHandle<Self>, V: ?Sized> {
    fn render_text_from(&mut self, v: &V) -> Text::Unmounted;
    fn update_text_from(&mut self, text: &mut Text, v: &V);
}

/// Trait alias for [`RenderTextFrom`] with all known primitive types.
pub trait RenderTextFromKnown<Text: UiHandle<Self>>:
    RenderTextFrom<Text, str>
    + RenderTextFrom<Text, i8>
    + RenderTextFrom<Text, u8>
    + RenderTextFrom<Text, i16>
    + RenderTextFrom<Text, u16>
    + RenderTextFrom<Text, i32>
    + RenderTextFrom<Text, u32>
    + RenderTextFrom<Text, i64>
    + RenderTextFrom<Text, u64>
    + RenderTextFrom<Text, i128>
    + RenderTextFrom<Text, u128>
    + RenderTextFrom<Text, isize>
    + RenderTextFrom<Text, usize>
    + RenderTextFrom<Text, f32>
    + RenderTextFrom<Text, f64>
    + RenderTextFrom<Text, char>
    + RenderTextFrom<Text, StringElement>
{
}

impl<R: ?Sized, Text: UiHandle<Self>> RenderTextFromKnown<Text> for R where
    R: RenderTextFrom<Text, str>
        + RenderTextFrom<Text, i8>
        + RenderTextFrom<Text, u8>
        + RenderTextFrom<Text, i16>
        + RenderTextFrom<Text, u16>
        + RenderTextFrom<Text, i32>
        + RenderTextFrom<Text, u32>
        + RenderTextFrom<Text, i64>
        + RenderTextFrom<Text, u64>
        + RenderTextFrom<Text, i128>
        + RenderTextFrom<Text, u128>
        + RenderTextFrom<Text, isize>
        + RenderTextFrom<Text, usize>
        + RenderTextFrom<Text, f32>
        + RenderTextFrom<Text, f64>
        + RenderTextFrom<Text, char>
        + RenderTextFrom<Text, StringElement>
{
}

pub trait Render: RenderWithContext {
    fn log(&mut self, v: &str);

    type CursorPlaceholder: 'static
        + UiHandle<Self>
        + crate::behaviors::NodeRenderSelf<Self>
        + crate::behaviors::NodeWithRenderContextAfterSelf<Self>
        + crate::behaviors::Node<Self>;
}

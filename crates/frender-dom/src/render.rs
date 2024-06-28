pub trait RenderTextFrom<Text, V: ?Sized> {
    /// should not move cursor
    fn render_text_from(&mut self, v: &V) -> Text;
    fn update_text_from(&mut self, text: &mut Text, v: &V);
}

/// Trait alias for [`RenderTextFrom`] with all known primitive types.
pub trait RenderTextFromKnown<Text>:
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
{
}

impl<R: ?Sized, Text> RenderTextFromKnown<Text> for R where
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
{
}

pub trait Render: RenderWithContext {
    fn log(&mut self, v: &str);

    type CursorPlaceholder: 'static
        + crate::behaviors::NodeRenderSelf<Self>
        + crate::behaviors::NodeWithRenderContextAfterSelf<Self>
        + crate::behaviors::Node<Self>;
}

pub trait RenderWithContext {
    type RenderContext<'a>: ?Sized + RenderContext<Renderer = Self>;
}

pub trait RenderContext {
    type Renderer: ?Sized + RenderWithContext;

    // TODO: remove
    fn map_mut_render_context<Res>(
        &mut self,
        f: impl FnOnce(&mut <Self::Renderer as RenderWithContext>::RenderContext<'_>) -> Res,
    ) -> Res;

    fn renderer_mut(&mut self) -> &mut Self::Renderer;
    fn log_cursor(&mut self);
    fn mark_cursor_skipped(&mut self);
}

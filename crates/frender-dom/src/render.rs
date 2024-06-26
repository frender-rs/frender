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

pub trait RenderAsText {
    fn render_as_text<R: ?Sized + RenderTextFromKnown<Text>, Text>(&self, renderer: &mut R)
        -> Text;

    fn render_as_text_update<R: ?Sized + RenderTextFromKnown<Text>, Text>(
        &self,
        renderer: &mut R,
        text: &mut Text,
    );
}

frender_common::impl_many!(
    impl<__> RenderAsText
        for each_of![
            str, //
            i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64, //
            char,
        ]
    {
        fn render_as_text<R: ?Sized + RenderTextFromKnown<Text>, Text>(
            &self,
            renderer: &mut R,
        ) -> Text {
            renderer.render_text_from(self)
        }

        fn render_as_text_update<R: ?Sized + RenderTextFromKnown<Text>, Text>(
            &self,
            renderer: &mut R,
            text: &mut Text,
        ) {
            renderer.update_text_from(text, self)
        }
    }
);

impl<S: std::borrow::Borrow<str>> RenderAsText for frender_common::TempStr<S> {
    fn render_as_text<R: ?Sized + RenderTextFromKnown<Text>, Text>(
        &self,
        renderer: &mut R,
    ) -> Text {
        str::render_as_text(self.borrow(), renderer)
    }

    fn render_as_text_update<R: ?Sized + RenderTextFromKnown<Text>, Text>(
        &self,
        renderer: &mut R,
        text: &mut Text,
    ) {
        str::render_as_text_update(self.borrow(), renderer, text)
    }
}

pub trait Render: RenderWithCursor + RenderWithContext {
    fn log(&mut self, v: &str);
}

pub trait RenderWithContext {
    type RenderContext<'a>;
}

// TODO: redesign renderer api with RenderContext
pub trait RenderWithCursor {
    type Cursor;

    fn cursor(&self) -> Self::Cursor;

    fn set_cursor(&mut self, cursor: Self::Cursor);

    /// Only for debugging
    fn cursor_skipped(&self) -> bool;

    /// Only for debugging
    fn set_cursor_skipped(&mut self, cursor_skipped: bool);

    fn set_cursor_by_ref(&mut self, cursor: &Self::Cursor);

    fn with_render_context<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        let cursor = self.cursor();
        let res = f(self);
        self.set_cursor(cursor);
        res
    }

    fn cursor_is_same_as(&self, other: &Self::Cursor) -> bool;

    fn log_cursor(&mut self);

    type CursorPlaceholder: 'static;

    /// Should add the placeholder to dom and move cursor after the placeholder
    fn cursor_placeholder_render(&mut self) -> Self::CursorPlaceholder;

    /// Should move the placeholder and move cursor after the placeholder
    fn cursor_placeholder_force_reposition(&mut self, cp: &mut Self::CursorPlaceholder);

    fn cursor_placeholder_unmount(&mut self, placeholder: &mut Self::CursorPlaceholder);

    fn move_cursor_after_placeholder(&mut self, placeholder: &mut Self::CursorPlaceholder);
}

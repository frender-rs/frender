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

pub trait RenderTextFrom<Text, V: ?Sized> {
    /// should not move cursor
    fn render_text_from(&mut self, v: &V) -> Text;
    fn update_text_from(&mut self, text: &mut Text, v: &V);
}

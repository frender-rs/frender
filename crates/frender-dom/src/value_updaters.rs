use crate::behaviors::HtmlTextAreaElement;

pub struct UpdateTextareaDefaultValue<'a, E: ?Sized + HtmlTextAreaElement<R>, R: ?Sized>(
    pub &'a mut E,
    pub &'a mut R,
);

impl<'a, E: ?Sized + HtmlTextAreaElement<R>, R: ?Sized> frender_html_common::ValueUpdater<str>
    for UpdateTextareaDefaultValue<'a, E, R>
{
    fn update(self, value: &str) {
        self.0.set_default_value(self.1, value)
    }

    fn remove(self) {
        self.0.set_default_value(self.1, "")
    }
}

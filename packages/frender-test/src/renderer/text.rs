use std::{borrow::Cow, rc::Rc, sync::Arc};

use frender_html::dom::{
    csr::{render::RenderTextFrom, UnmountedUiHandle},
    string_element::StringElement,
};
use frender_macro_rules::impl_many;
use frender_reactive_value::{static_or_temp_ref::StaticOrTempRef, temp_ref::TempRef};

use crate::text::{Text, UnmountedText};

use super::Renderer;

trait KnownToString: ToString {}

impl_many!(
    impl<__> KnownToString
        for each_of![
            &'static str,
            StringElement,
            String,
            Cow<'static, str>,
            Rc<str>,
            Arc<str>,
            i8,
            u8,
            i16,
            u16,
            i32,
            u32,
            i64,
            u64,
            i128,
            u128,
            isize,
            usize,
            f32,
            f64,
            char,
        ]
    {
    }
);

trait CustomIntoString {
    fn custom_into_string(self) -> String;
}

impl<T: KnownToString> CustomIntoString for T {
    fn custom_into_string(self) -> String {
        self.to_string()
    }
}

impl CustomIntoString for StaticOrTempRef<'_, str> {
    fn custom_into_string(self) -> String {
        self.as_ref().to_owned()
    }
}

impl_many!(
    impl<__> CustomIntoString
        for each_of![
            //
            TempRef<'_, str>,
            TempRef<'_, Rc<str>>,
            TempRef<'_, Arc<str>>,
        ]
    {
        fn custom_into_string(self) -> String {
            str::to_owned(self.0)
        }
    }
);

impl CustomIntoString for TempRef<'_, StringElement> {
    fn custom_into_string(self) -> String {
        self.0.to_string()
    }
}

impl<T: CustomIntoString> RenderTextFrom<T> for Renderer {
    type Text = Text;

    fn render_text_from(render_context: &mut Self::RenderContext<'_>, v: T) -> Self::Text {
        UnmountedText::new(v.custom_into_string()).mount(render_context)
    }

    fn update_text_from(&mut self, text: &mut Self::Text, v: T) {
        text.update(v.custom_into_string());
    }
}

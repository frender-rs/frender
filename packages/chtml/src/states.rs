use super::input_stream::BufferedPreprocessedInputStream;

pub(crate) mod before_attribute_name;
pub(crate) mod before_attribute_value;

pub(crate) mod attribute_name;
pub(crate) mod attribute_value;

pub(crate) mod after_attribute_name;
pub(crate) mod after_attribute_value;

mod character_reference;

pub struct SelfClosingStartTag<'a>(BufferedPreprocessedInputStream<'a>);
mod self_closing_start_tag;

pub struct Data<'a>(BufferedPreprocessedInputStream<'a>);

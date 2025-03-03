mod content_editable;
pub use content_editable::AttrKindOfContentEditable;

mod spellcheck;
pub use spellcheck::Spellcheck;

fn bool_to_str(this: bool) -> &'static str {
    if this {
        "true"
    } else {
        "false"
    }
}

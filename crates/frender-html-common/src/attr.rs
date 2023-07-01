pub enum HtmlAttributeValue<S> {
    String(S),
    /// If a boolean attribute is present, its value is true, and if it's absent, its value is false.
    ///
    /// HTML defines restrictions on the allowed values of boolean attributes. Please see
    /// [boolean_attributes]
    ///
    /// [boolean_attributes]: https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes#boolean_attributes
    BooleanTrue,
}

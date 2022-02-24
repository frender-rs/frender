use convert_js::ToJs;

#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union, rename_all = "camelCase")]
pub enum Inheritable<T> {
    Value(T),
    Inherit,
}

/// Hints at the type of data that might be entered by the user while editing the element or its contents
///
/// @see https://html.spec.whatwg.org/multipage/interaction.html#input-modalities:-the-inputmode-attribute
///
/// 'none' | 'text' | 'tel' | 'url' | 'email' | 'numeric' | 'decimal' | 'search'
#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union, rename_all = "camelCase")]
pub enum HtmlInputMode {
    None,
    Text,
    Tel,
    Url,
    Email,
    Numeric,
    Decimal,
    Search,
}

#[derive(Debug, Clone, ToJs)]
#[convert_js(union)]
pub enum AnchorTarget<'a> {
    #[convert_js(rename = "_self")]
    SelfTarget,
    #[convert_js(rename = "_blank")]
    Blank,
    #[convert_js(rename = "_parent")]
    Parent,
    #[convert_js(rename = "_top")]
    Top,
    Custom(&'a str),
}

#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union, rename_all = "kebab-case")]
pub enum ReferrerPolicy {
    #[convert_js(rename = "")]
    None,
    NoReferrer,
    NoReferrerWhenDowngrade,
    Origin,
    OriginWhenCrossOrigin,
    SameOrigin,
    StrictOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
}

#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union, rename_all = "lowercase")]
pub enum ButtonType {
    Submit,
    Reset,
    Button,
}

#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union, rename_all = "lowercase")]
pub enum Loading {
    Eager,
    Lazy,
}

/// `"anonymous" | "use-credentials" | ""`
#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union, rename_all = "kebab-case")]
pub enum CrossOrigin {
    #[convert_js(rename = "")]
    None,
    Anonymous,
    UseCredentials,
}

/// `"async" | "auto" | "sync"`
#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union, rename_all = "lowercase")]
pub enum Decoding {
    Async,
    Auto,
    Sync,
}

#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union, rename_all = "kebab-case")]
pub enum InputType<'a> {
    Button,
    Checkbox,
    Color,
    Date,
    DatetimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    Text,
    Time,
    Url,
    Week,
    Custom(&'a str),
}

/// `'enter' | 'done' | 'go' | 'next' | 'previous' | 'search' | 'send'`
#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union, rename_all = "lowercase")]
pub enum EnterKeyHint {
    Enter,
    Done,
    Go,
    Next,
    Previous,
    Search,
    Send,
}

/// `'1' | 'a' | 'A' | 'i' | 'I'`
#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union)]
pub enum OListType {
    /// `1` for numbers (default)
    #[convert_js(rename = "1")]
    Number,
    /// `a` for lowercase letters
    #[convert_js(rename = "a")]
    LetterLower,
    /// `A` for uppercase letters
    #[convert_js(rename = "A")]
    LetterUpper,
    /// `i` for lowercase Roman numerals
    #[convert_js(rename = "i")]
    RomanLower,
    /// `I` for uppercase Roman numerals
    #[convert_js(rename = "I")]
    RomanUpper,
}

/// `"left" | "center" | "right" | "justify" | "char"`
#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union, rename_all = "lowercase")]
pub enum TableCellAlign {
    Left,
    Center,
    Right,
    Justify,
    Char,
}

/// `"top" | "middle" | "bottom" | "baseline"`
#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union, rename_all = "lowercase")]
pub enum TableCellVAlign {
    Top,
    Middle,
    Bottom,
    Baseline,
}

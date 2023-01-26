use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

fn colon2_with_span(span: Span) -> (Punct, Punct) {
    let mut a = Punct::new(':', Spacing::Joint);
    let mut b = Punct::new(':', Spacing::Alone);
    a.set_span(span);
    b.set_span(span);
    (a, b)
}

/// ```txt
/// [crate::rsx_xml]
/// [<]
/// div />
/// ```
///
/// will be transformed to:
///
/// ```plain
/// crate::rsx_xml! {
///     < self::intrinsic_components::div />
/// }
/// ```
#[inline]
pub fn auto_intrinsic(input: TokenStream) -> TokenStream {
    let mut input = input.into_iter();
    let macro_path = if let Some(TokenTree::Group(group)) = input.next() {
        group.stream()
    } else {
        panic!("expect grouped macro path")
    };

    let prepend_ts = if let Some(TokenTree::Group(group)) = input.next() {
        group.stream()
    } else {
        panic!("expect grouped token stream to prepend")
    };

    let element_path = if let Some(TokenTree::Ident(ident)) = input.next() {
        let first_char = ident.to_string().chars().nth(0).unwrap();
        if first_char >= 'a' && first_char <= 'z' {
            let span = ident.span();
            let (colon1, colon2) = colon2_with_span(span);
            TokenStream::from_iter([
                TokenTree::Ident(Ident::new("self", span)),
                colon1.clone().into(),
                colon2.clone().into(),
                TokenTree::Ident(Ident::new("intrinsic_components", span)),
                colon1.into(),
                colon2.into(),
                ident.into(),
            ])
        } else {
            TokenTree::Ident(ident).into()
        }
    } else {
        panic!("expect ident")
    };

    TokenStream::from_iter([
        macro_path,
        TokenTree::Punct(Punct::new('!', Spacing::Alone)).into(),
        TokenTree::Group(Group::new(Delimiter::Brace, {
            let mut ts = TokenStream::from_iter([prepend_ts, element_path]);
            ts.extend(input);
            ts
        }))
        .into(),
    ])
}

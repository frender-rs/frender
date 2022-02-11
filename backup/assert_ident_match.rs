/// `{ MyComp } { MyComp }`
pub struct AssertRsxIdentMatchArgs {
    brace1: syn::token::Brace,
    expected: syn::Ident,
    brace2: syn::token::Brace,
    got: Option<syn::Ident>,
}

impl Parse for AssertRsxIdentMatchArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expected;
        let got;
        Ok(AssertRsxIdentMatchArgs {
            brace1: braced! (expected in input),
            expected: expected.parse()?,
            brace2: braced!(got in input),
            got: got.parse()?,
        })
    }
}

pub fn assert_rsx_ident_match(args: AssertRsxIdentMatchArgs) -> syn::Result<()> {
    let AssertRsxIdentMatchArgs { expected, got, .. } = args;

    if let Some(got) = got {
        if got != expected {
            let span_expected = expected.span();
            let span_got = got.span();

            let mut err = syn::Error::new(
                span_got,
                format!("Expect </{}> or </>, but got </{}>.", expected, got),
            );
            err.combine(syn::Error::new(
                span_expected,
                format!("<{}> not enclosed properly.", expected),
            ));
            return Err(err);
        }
    }
    Ok(())
}

#[proc_macro]
pub fn assert_rsx_element_start_end_ident_match(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as rsx::AssertRsxIdentMatchArgs);
    if let Err(err) = rsx::assert_rsx_ident_match(args) {
        err.into_compile_error().into()
    } else {
        TokenStream::new()
    }
}

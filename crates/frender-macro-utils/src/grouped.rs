#[derive(Clone)]
pub struct Grouped<G, S> {
    pub group_token: G,
    pub content: S,
}

macro_rules! impl_grouped {
    (
        $(
            $name:ident
            (
                $ty:ty ,
                $($macro_path:tt)+
            )
        ),*
        $(,)?
    ) => {$(
        pub type $name<S> = Grouped<$ty, S>;

        pub fn $name<S>(content: S) -> $name<S> {
            $name {
                group_token: Default::default(),
                content,
            }
        }

        impl<S: ::syn::parse::Parse> $name<S> {
            pub fn parse_many(input: syn::parse::ParseStream) -> syn::Result<Vec<Self>> {
                let mut res = vec![];

                while let Some(v) = input.parse()? {
                    res.push(v)
                }

                Ok(res)
            }
        }

        impl<S: ::syn::parse::Parse> ::syn::parse::Parse for $name<S> {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                let content;
                Ok(Self {
                    group_token: $($macro_path)+ !(content in input),
                    content: content.parse()?,
                })

            }
        }

        impl<S> ::syn::token::CustomToken for $name<S> {
            fn peek(cursor: syn::buffer::Cursor) -> bool {
                <$ty as syn::token::Token>::peek(cursor)
            }

            fn display() -> &'static str {
                <$ty as syn::token::Token>::display()
            }
        }
    )*};
}

impl_grouped! {
    Parenthesized(syn::token::Paren, syn::parenthesized),
    Bracketed(syn::token::Bracket, syn::bracketed),
    Braced(syn::token::Brace, syn::braced),
}

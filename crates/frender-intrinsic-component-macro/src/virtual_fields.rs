use syn::parse::Parse;

use crate::utils::grouped::{Braced, Bracketed};

use super::{Fields, IntrinsicComponentProps};

#[derive(Clone)]
pub struct IntrinsicComponentPropsVirtual {
    pub virtual_token: syn::Token![virtual],
    pub fields: Braced<Fields>,
    pub inherits: Vec<Bracketed<IntrinsicComponentProps>>,
}
impl IntrinsicComponentPropsVirtual {
    pub fn try_unzip(self) -> syn::Result<Vec<super::IntrinsicComponentPropsData>> {
        if self.inherits.is_empty() {
            Err(syn::Error::new(
                self.virtual_token.span,
                "virtual fields should have at least one inherited struct",
            ))
        } else {
            let prepend_fields = self.fields.content;
            let mut res = vec![];
            for inherit in self.inherits {
                match inherit.content {
                    IntrinsicComponentProps::Virtual(mut v) => {
                        v.fields.content.prepend(prepend_fields.clone());
                        res.extend(v.try_unzip()?);
                    }
                    IntrinsicComponentProps::Data(mut d) => {
                        d.fields.content.prepend(prepend_fields.clone());
                        res.push(d);
                    }
                }
            }

            Ok(res)
        }
    }
}

impl Parse for IntrinsicComponentPropsVirtual {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            virtual_token: input.parse()?,
            fields: input.parse()?,
            inherits: input.call(Bracketed::parse_many)?,
        })
    }
}

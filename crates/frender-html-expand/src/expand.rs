use syn::visit_mut::VisitMut;

pub struct ExpandIntrinsicComponentMacro {
    attribute_for_identifying: syn::Attribute,
    result: Result<Vec<syn::Item>, syn::Error>,
}

impl ExpandIntrinsicComponentMacro {
    pub fn new() -> Self {
        Self {
            attribute_for_identifying: syn::parse_quote!(#[cfg(not(frender_macro_def_intrinsic_component_props))]),
            result: Ok(vec![]),
        }
    }

    pub fn finish(self) -> syn::Result<Vec<syn::Item>> {
        self.result
    }
}

fn parse_items(input: syn::parse::ParseStream) -> syn::Result<Vec<syn::Item>> {
    let mut out = vec![];
    while !input.is_empty() {
        out.push(input.parse()?);
    }

    Ok(out)
}

impl VisitMut for ExpandIntrinsicComponentMacro {
    fn visit_item_mut(&mut self, item: &mut syn::Item) {
        let items = if let Ok(items) = &mut self.result {
            items
        } else {
            return;
        };

        let item_macro = match item {
            syn::Item::Macro(m) => m,
            _ => return,
        };

        let tokens = if item_macro.attrs.len() == 1
            && item_macro.attrs[0] == self.attribute_for_identifying
        {
            std::mem::take(&mut item_macro.mac.tokens)
        } else {
            return;
        };

        *item = syn::Item::Verbatim(Default::default());

        match syn::parse2(tokens)
            .map(frender_intrinsic_component_macro::into_ts)
            .and_then(|input| syn::parse::Parser::parse2(parse_items, input))
        {
            Ok(ref mut expanded_items) => items.append(expanded_items),
            Err(error) => self.result = Err(error),
        }
    }
}

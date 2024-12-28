use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use quote::ToTokens;
use syn::parse_quote;

mod utils;

thread_local!(
    static SPECIAL_ATTR_META: syn::Meta = parse_quote!(cfg(feature = "macros_not_expanded"));
);

/// Returns true if the attribute is `cfg(feature = "macros_not_expanded")`
fn is_special_attribute(attr: &syn::Attribute) -> bool {
    SPECIAL_ATTR_META.with(|meta| attr.meta == *meta)
}

#[test]
fn test_is_special_attribute() {
    use syn::parse::Parser as _;

    let tests: &[&[syn::Attribute]] = &[
        &[parse_quote!(#[cfg(feature = "macros_not_expanded")])],
        &syn::Attribute::parse_outer
            .parse_str(r#"#[cfg(feature = "macros_not_expanded")]"#)
            .unwrap(),
        &syn::Attribute::parse_outer
            .parse_str(r#"#[cfg(feature="macros_not_expanded")]"#)
            .unwrap(),
    ];

    for test in tests {
        let [attr] = test else { unreachable!() };
        SPECIAL_ATTR_META.with(|meta| assert_eq!(attr.meta, *meta));
        assert!(is_special_attribute(attr));
    }
}

fn expand_and_write(src_root: &Path) -> io::Result<()> {
    let items = utils::cargo_expand_html("frender-html", "html")?;

    let parent_folder = src_root.join("html");

    for item in items {
        let syn::Item::Mod(item) = item else {
            continue;
        };

        let syn::ItemMod {
            mut attrs,
            vis: _,
            unsafety,
            mod_token: _,
            ident,
            content,
            semi,
        } = item;

        let mut has_special_attribute = false;

        attrs.retain(|attr| {
            // remove the special attribute
            if is_special_attribute(attr) {
                has_special_attribute = true;
                return false;
            }

            // only keeps inner attributes
            matches!(attr.style, syn::AttrStyle::Inner(_))
        });

        if !has_special_attribute {
            continue;
        }

        assert!(unsafety.is_none());
        assert!(semi.is_none());

        let Some((_, content)) = content else {
            unreachable!()
        };

        let mod_name = ident.to_string();

        write_mod_content_into_dir(
            //
            &parent_folder,
            &mod_name,
            attrs,
            content,
            0,
        )?;
    }

    return Ok(());
}

fn main() -> io::Result<()> {
    let workspace_root = utils::locate_cargo_workspace_root()?;
    let src_root = workspace_root.join("packages/frender-html/src");

    expand_and_write(&src_root)?;

    // Ok(())
    // run twice
    utils::cargo_fmt_package("frender-html")?;
    utils::cargo_fmt_package("frender-html")
}

fn write_mod_content_into_dir(
    mod_root_dir: &Path,
    mod_name: &str,
    inner_attrs: Vec<syn::Attribute>,
    items: Vec<syn::Item>,
    unwrap_mod_depth: u8,
) -> io::Result<()> {
    let mod_dir = mod_root_dir.join(mod_name);
    let mut mod_file = mod_root_dir.join(format!("{mod_name}.rs"));
    if mod_dir.exists() {
        fs::remove_dir_all(&mod_dir)?;
    }
    if mod_file.exists() {
        fs::remove_file(&mod_file)?;
    }

    if unwrap_mod_depth > 0 {
        fs::create_dir_all(&mod_dir)?;
        mod_file = mod_dir.join("mod.rs");
    };

    let mut file = fs::File::create(&mod_file)?;

    for attr in inner_attrs {
        assert!(matches!(&attr.style, syn::AttrStyle::Inner(_)));
        file.write_all(attr.into_token_stream().to_string().as_bytes())?;
    }

    for item in items {
        match item {
            syn::Item::Mod(syn::ItemMod {
                attrs,
                vis,
                unsafety: _,
                mod_token: _,
                ident,
                content: Some((_, items)),
                semi: None,
            }) if unwrap_mod_depth > 0 && !items.is_empty() => {
                let (outer_attrs, inner_attrs) =
                    attrs
                        .into_iter()
                        .partition::<Vec<_>, _>(|attr| match &attr.style {
                            syn::AttrStyle::Outer => true,
                            syn::AttrStyle::Inner(_) => false,
                        });

                for attr in outer_attrs {
                    file.write_all(attr.into_token_stream().to_string().as_bytes())?;
                }

                write!(file, "{} mod {};", vis.into_token_stream(), ident,)?;

                write_mod_content_into_dir(
                    &mod_dir,
                    &ident.to_string(),
                    inner_attrs,
                    items,
                    unwrap_mod_depth - 1,
                )?;
            }
            _ => {
                // directly append to mod.rs
                file.write_all(utils::format_item(item).as_bytes())?;
            }
        }
    }

    Ok(())
}

use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use quote::ToTokens;

mod utils;

fn main() -> io::Result<()> {
    let workspace_root = utils::locate_cargo_workspace_root()?;
    let src_root = workspace_root.join("crates/frender-html/src");

    let code = utils::cargo_expand_html("frender-html", "html::props_builders")?;

    assert!(code.shebang.is_none());

    write_mod_content_into_dir(
        &src_root.join("html"),
        "props_builders",
        code.attrs,
        code.items,
        1,
    )?;

    Ok(())
    // utils::cargo_fmt_package("frender-html")
}

pub fn write_mod_content_into_dir(
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

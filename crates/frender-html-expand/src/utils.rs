use std::{
    io::{self, Write},
    path::PathBuf,
};

use syn::visit_mut::VisitMut;

pub fn format_item(item: syn::Item) -> String {
    prettyplease::unparse(&syn::File {
        shebang: None,
        attrs: vec![],
        items: vec![item],
    })
}

pub fn cargo_expand_html(pkg_name: &str, mod_path: &str) -> io::Result<syn::File> {
    let output = std::process::Command::new("cargo")
        .arg("expand")
        .arg("-p")
        .arg(pkg_name)
        .arg(mod_path)
        .arg("--features")
        .arg("props_builders_not_expanded")
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to run cargo expand".to_string(),
        ));
    }

    io::stderr().write_all(&output.stderr)?;

    let output = output.stdout;
    let code = string_from_utf8(output)?;
    let code = code.replace("pub(crate) use impl_props_builder_fns;", "");
    let syn::ItemMod {
        attrs: _,
        vis: _,
        mod_token: _,
        ident: _,
        content,
        semi,
    } = syn::parse_str(&code).map_err(io_error_other)?;

    assert!(semi.is_none());
    let (_, items) = content.unwrap();

    Ok(syn::File {
        shebang: None,
        attrs: vec![],
        items,
    })
}

pub fn locate_cargo_workspace_root() -> io::Result<PathBuf> {
    let output = std::process::Command::new("cargo")
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()?
        .stdout;

    let mut path = PathBuf::from(string_from_utf8(output)?.trim());
    path.pop();
    Ok(path)
}

pub fn string_from_utf8(utf8: Vec<u8>) -> Result<String, io::Error> {
    let s = String::from_utf8(utf8).map_err(io_error_other)?;
    Ok(s)
}

pub fn io_error_other<E: Into<Box<dyn std::error::Error + Send + Sync>>>(error: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, error)
}

pub fn cargo_fmt_package(package: &str) -> io::Result<()> {
    let output = std::process::Command::new("cargo")
        .arg("fmt")
        .arg("-p")
        .arg(package)
        .output()?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to run cargo fmt -p {package}"),
        ))
    }
}

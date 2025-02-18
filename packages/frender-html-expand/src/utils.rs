use std::{
    io::{self, Write},
    path::PathBuf,
};

pub fn format_item(item: syn::Item) -> String {
    prettyplease::unparse(&syn::File {
        shebang: None,
        attrs: vec![],
        items: vec![item],
    })
}

pub fn cargo_expand_html(
    pkg_name: &str,
    mod_name: &str,
    features: &str,
) -> io::Result<Vec<syn::Item>> {
    let mut command = std::process::Command::new("cargo");
    command
        .arg("expand")
        .arg("-p")
        .arg(pkg_name)
        .arg("--features")
        .arg(features)
        .arg("--ugly");

    eprintln!(
        "Run: {} {}",
        command.get_program().to_string_lossy(),
        command
            .get_args()
            .map(|a| a.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ")
    );

    let output = command.output()?;

    if !output.status.success() {
        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to run cargo expand".to_string(),
        ));
    }

    io::stderr().write_all(&output.stderr)?;

    let output = output.stdout;
    let code = string_from_utf8(output)?;
    let syn::File {
        shebang: _,
        attrs: _,
        items,
    } = syn::parse_str(&code).unwrap();

    let item = items
        .into_iter()
        .find_map(|item| match item {
            syn::Item::Mod(item) if item.ident == mod_name => Some(item),
            _ => None,
        })
        .expect(&format!("doesn't found mod {mod_name}"));

    let syn::ItemMod {
        attrs: _,
        vis: _,
        unsafety: _,
        mod_token: _,
        ident: _,
        content,
        semi,
    } = item;

    assert!(semi.is_none());
    let (_, items) = content.unwrap();

    Ok(items)
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

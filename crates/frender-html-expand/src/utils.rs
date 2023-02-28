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

pub fn cargo_expand_html(file_path: impl AsRef<std::path::Path>) -> io::Result<syn::File> {
    let content = std::fs::read_to_string(file_path)?;
    let mut code = syn::parse_file(&content).expect("file content is valid rust code");
    let mut visitor = crate::expand::ExpandIntrinsicComponentMacro::new();
    visitor.visit_file_mut(&mut code);
    let ref mut items = visitor.finish().map_err(crate::utils::io_error_other)?;
    code.items.append(items);
    Ok(code)
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

pub fn io_error_other<E: std::error::Error + Send + Sync + 'static>(error: E) -> io::Error {
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

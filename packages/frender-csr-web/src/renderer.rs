#[cfg(feature = "csr")]
mod csr;

pub struct Renderer {
    document: web_sys::Document,
}

pub struct RendererWithRoot {
    renderer: Renderer,
    root: web_sys::Element,
}

impl RendererWithRoot {
    pub fn new(document: web_sys::Document, root_parent: web_sys::Element) -> Self {
        Self {
            renderer: Renderer { document },
            root: root_parent,
        }
    }
}

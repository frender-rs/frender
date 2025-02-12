mod parent_only;

pub mod inner_html {
    pub mod csr;
}
pub mod input;
pub mod script {
    pub mod csr;
    mod ssr;
}
pub mod style {
    pub mod csr;
    mod ssr;
}
pub mod textarea {
    pub mod csr;
    mod props_builder;
    pub mod ssr;
}
mod void_elements;

mod data_types;
mod html_common_shared_props;
mod html_media_shared_props;

pub use data_types::*;
pub use html_common_shared_props::*;
pub use html_media_shared_props::*;

pub(crate) mod macros;

pub mod css;
pub mod html_components;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

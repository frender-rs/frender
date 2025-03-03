#[cfg(feature = "csr")]
pub(super) mod csr;
#[cfg(feature = "ssr")]
pub(super) mod ssr;

mod imp;

pub trait InputType {
    type InputTypeStr;

    fn maybe_into_input_type_str(this: Self) -> Option<Self::InputTypeStr>;
}

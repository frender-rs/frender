mod value_kind;
pub use value_kind::InputValueKind;

mod checked;
mod r#type;
mod value;

pub use {checked::InputChecked, r#type::InputType, value::InputValue};

#[cfg(feature = "csr")]
pub use {checked::csr::CsrInputChecked, r#type::csr::CsrInputType, value::csr::CsrInputValue};
#[cfg(feature = "ssr")]
pub use {checked::ssr::SsrInputChecked, r#type::ssr::SsrInputType, value::ssr::SsrInputValue};

mod data_model;
mod into_data_model;

pub use {data_model::InputDataModel, into_data_model::IntoInputDataModel};

#[cfg(feature = "csr")]
pub use into_data_model::csr::IntoCsrInputDataModel;
#[cfg(feature = "ssr")]
pub use into_data_model::ssr::IntoSsrInputDataModel;

#[cfg(feature = "csr")]
mod csr;

#[cfg(feature = "csr")]
pub use csr::InputElement;

#[cfg(feature = "ssr")]
mod ssr;

#[cfg(feature = "web")]
pub(crate) mod web;

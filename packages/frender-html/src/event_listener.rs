use std::marker::PhantomData;

#[cfg(feature = "csr")]
mod csr;

pub struct Property<EVT, F> {
    _event_type: PhantomData<EVT>,
    f: F,
}

impl<EVT, F> Property<EVT, F> {
    pub(crate) fn new(f: F) -> Self {
        Self { _event_type: PhantomData, f }
    }
}

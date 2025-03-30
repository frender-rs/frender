use frender_common::define_phantom_wrapper;

define_phantom_wrapper!(
    #[always_derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ConstValue<T: ?Sized + HasConstValue>;
);

pub trait HasConstValue {
    type Value;
    const VALUE: Self::Value;
}

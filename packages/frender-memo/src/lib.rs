use frender_fn_traits::FnOnce2;

#[cfg(feature = "csr")]
mod csr;

#[cfg(feature = "csr")]
#[cfg(feature = "experimental")]
pub mod csr_experimental {
    pub use crate::csr::{CompoundState, Kind};
}

#[cfg(feature = "ssr")]
mod ssr;

pub struct Memo<F, Dep>(pub F, pub Dep);

pub struct MemoAndProvideFirstArgument<F: for<'a> FnOnce2<A, &'a Dep>, A, Dep>(
    pub F,
    pub A,
    pub Dep,
);

impl<F, A, Dep> MemoAndProvideFirstArgument<F, A, Dep>
where
    F: for<'a> FnOnce2<A, &'a Dep>,
{
    fn into_f_and_dep(self) -> (impl FnOnce(&Dep) -> <F as FnOnce2<A, &Dep>>::Output_, Dep) {
        let Self(f, arg, dep) = self;

        (move |dep| f(arg, dep), dep)
    }

    pub fn into_memo(self) -> Memo<impl FnOnce(&Dep) -> <F as FnOnce2<A, &Dep>>::Output_, Dep> {
        let (f, dep) = self.into_f_and_dep();
        Memo(f, dep)
    }
}

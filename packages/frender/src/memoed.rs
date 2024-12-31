use crate::fn_traits::{FnOnce1, FnOnce2};

pub use self::Memo as memo;

pub mod csr;
mod ssr;

pub struct Memo<F, Dep>(pub F, pub Dep);

impl<F, Dep> Memo<F, Dep> {
    // The third argument of `run` is `new_dep == memoed_dep`.
    // `true` means new dep equals to the memoed dep.
    fn map_memoed<R>(
        self,
        memoed_dep: &mut Dep,
        eq: impl FnOnce(&Dep, &Dep) -> bool,
        run: impl FnOnce(F, &mut Dep, bool) -> R,
    ) -> R {
        let Self(f, dep) = self;

        let dep_eq_memoed_dep;
        if eq(memoed_dep, &dep) {
            *memoed_dep = dep; // TODO: should we update memoed dep even if the new dep eq the memoed dep?
            dep_eq_memoed_dep = true;
        } else {
            *memoed_dep = dep;
            dep_eq_memoed_dep = false;
        }

        run(f, memoed_dep, dep_eq_memoed_dep)
    }
}

/// As [`CsrElement`](crate::CsrElement), this type has the same
/// [`RenderStateKind`](crate::CsrElement::RenderStateKind) as [`Memo<F, Dep>`].
///
/// It will always try to update render state with the element returned by `F` without the memoed dep updated.
pub struct MemoPhantom<F: for<'a> FnOnce1<&'a Dep>, Dep> {
    pub f: F,
    _dep: std::marker::PhantomData<Dep>,
}

impl<F: for<'a> FnOnce1<&'a Dep>, Dep> MemoPhantom<F, Dep> {
    pub const fn new(f: F) -> Self {
        Self {
            f,
            _dep: std::marker::PhantomData,
        }
    }
}

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

/// Works like [`MemoPhantom`]
pub struct MemoPhantomAndProvideFirstArgument<F: for<'a> FnOnce2<A, &'a Dep>, A, Dep> {
    pub f: F,
    pub first_argument: A,
    _dep: std::marker::PhantomData<Dep>,
}

impl<F: for<'a> FnOnce2<V, &'a Dep>, V, Dep> MemoPhantomAndProvideFirstArgument<F, V, Dep> {
    pub const fn new(f: F, first_argument: V) -> Self {
        Self {
            f,
            first_argument,
            _dep: std::marker::PhantomData,
        }
    }

    fn into_f(self) -> impl FnOnce(&Dep) -> <F as FnOnce2<V, &Dep>>::Output_ {
        move |dep: &_| (self.f)(self.first_argument, dep)
    }

    pub fn into_memo_phantom(
        self,
    ) -> MemoPhantom<impl FnOnce(&Dep) -> <F as FnOnce2<V, &Dep>>::Output_, Dep> {
        MemoPhantom::new(self.into_f())
    }
}

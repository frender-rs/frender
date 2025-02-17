use crate::fn_traits::FnOnce2;

pub mod csr;
mod ssr;

pub struct Memo<F, Dep>(pub F, pub Dep);

impl<F, Dep> Memo<F, Dep> {
    fn map_changed_memo<Out>(
        self,
        memoed_dep: &mut Dep,
        ne: impl FnOnce(&Dep, &Dep) -> bool,
        run: impl FnOnce(F, &mut Dep) -> Out,
    ) -> Option<Out> {
        let Self(f, dep) = self;

        let changed = ne(memoed_dep, &dep);

        *memoed_dep = dep; // TODO: should we update memoed dep even if the new dep eq the memoed dep?

        changed.then(|| run(f, memoed_dep))
    }
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

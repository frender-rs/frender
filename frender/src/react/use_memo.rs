use std::{any::Any, cell::RefCell, rc::Rc};

use super::IntoRc;

pub fn use_memo_no_dep<T: Any, R: IntoRc<T>, F: FnOnce() -> R>(func: F) -> Rc<T> {
    let mut func = super::utils::fn_once_in_runtime(move || {
        let rc = func().into_rc();
        let k = forgotten::forget_rc(rc).into_shared();
        let k = k.as_usize();
        *k
    });

    let obj = react_sys::use_ref_usize_with(&mut func);

    let k = obj.current();

    let v = forgotten::try_get(&unsafe { forgotten::SharedForgottenKey::<T>::from_usize(k) })
        .expect("use memo ptr should not be freed before element unmounted");

    super::use_effect_on_mounted(move || {
        move || {
            // SAFETY: k is valid and will never change in the lifetime of the component
            unsafe { forgotten::try_free_with_usize(k) };
        }
    });

    v
}

pub fn use_memo_one<
    D: 'static + PartialEq,
    T: 'static,
    RD: IntoRc<D>,
    RT: IntoRc<T>,
    F: FnOnce(&Rc<D>) -> RT,
>(
    func: F,
    dep: RD,
) -> Rc<T> {
    let dep: Rc<D> = dep.into_rc();
    let dep_and_value =
        use_memo_no_dep::<RefCell<Option<(Rc<D>, Rc<T>)>>, _, _>(|| RefCell::new(None));

    let mut dep_and_value = dep_and_value.as_ref().borrow_mut();

    match &*dep_and_value {
        Some(t) if &t.0 == &dep => {
            // dep not changed
            Rc::clone(&t.1)
        }
        _ => {
            // dep changed
            let new_v: Rc<T> = func(&dep).into_rc();

            *dep_and_value = Some((dep, new_v.clone()));

            new_v
        }
    }
}

#[macro_export]
macro_rules! use_memo {
    ( $(<$t:ty>)? | $($dep:ident),* | $e:expr ) => {{
        $crate::react::use_memo()
    }};
}

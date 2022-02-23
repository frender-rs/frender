use std::rc::Rc;

use wasm_bindgen::UnwrapThrowExt;

use super::ReadRef;

#[derive(Debug)]
pub struct ReadRefRc<T: ?Sized>(pub Rc<T>);

impl<T: ?Sized> Clone for ReadRefRc<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T: ?Sized> PartialEq for ReadRefRc<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: ?Sized> Eq for ReadRefRc<T> {}

impl<T: ?Sized> ReadRef<Rc<T>> for ReadRefRc<T> {
    #[inline]
    fn current(&self) -> Rc<T> {
        Rc::clone(&self.0)
    }
}

pub fn use_ref_readonly<T: 'static + ?Sized>(initial_rc: Rc<T>) -> ReadRefRc<T> {
    use_ref_readonly_with(move || initial_rc)
}

pub fn use_ref_readonly_with<T: 'static + ?Sized, F: FnOnce() -> Rc<T>>(
    get_initial_rc: F,
) -> ReadRefRc<T> {
    let obj = react_sys::use_ref_optional_usize(None);

    let k = obj.current();

    let (k, v) = if let Some(k) = k {
        let v = unsafe { forgotten::try_get_with_usize::<Rc<T>>(&k) };
        let v =
            v.expect_throw("use_ref_readonly ptr is expected to be valid before element unmounted");
        let v = Rc::clone(v.as_ref());
        (k, v)
    } else {
        let rc: Rc<T> = get_initial_rc();
        let k = forgotten::forget(Rc::clone(&rc));
        let k = *k.into_shared().as_usize();
        obj.set_current(Some(k));
        (k, rc)
    };

    // SAFETY: k will never change in the lifetime of the component
    crate::use_effect_on_mounted(move || {
        move || {
            // SAFETY: k is valid and will never change in the lifetime of the component
            unsafe { forgotten::try_free_with_usize(k) };
        }
    });

    ReadRefRc(v)
}

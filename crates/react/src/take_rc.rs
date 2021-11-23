use std::rc::Rc;

pub trait IntoRc<T: ?Sized> {
    fn into_rc(self) -> Rc<T>;
}

impl<T: ?Sized> IntoRc<T> for Rc<T> {
    fn into_rc(self) -> Rc<T> {
        self
    }
}

impl<T> IntoRc<T> for T {
    fn into_rc(self) -> Rc<T> {
        Rc::new(self)
    }
}

pub trait TakeRc<T> {
    fn take_rc(self) -> Rc<T>;
}

impl<T, R: IntoRc<T>, F: FnOnce() -> R> TakeRc<T> for F {
    fn take_rc(self) -> Rc<T> {
        self().into_rc()
    }
}

impl<T> TakeRc<T> for Rc<T> {
    fn take_rc(self) -> Rc<T> {
        self
    }
}

// // Do we need to impl TakeInitialRc for common types ?
// macro_rules! impl_take_for_types {
//     ($($t:ty),+ $(,)?) => {
//         $(

//             impl TakeInitialRc<$t> for $t {
//                 fn take_initial_rc(self) -> Rc<$t> {
//                     Rc::new(self)
//                 }
//             }
//         )+
//     };
// }

// impl_take_for_types! {
//     String,
//     usize,
// }

pub trait IntoOptionalRc<T: ?Sized> {
    fn into_optional_rc(self) -> Option<Rc<T>>;
}

impl<T: ?Sized> IntoOptionalRc<T> for Rc<T> {
    fn into_optional_rc(self) -> Option<Rc<T>> {
        Some(self)
    }
}

impl<T> IntoOptionalRc<T> for Option<T> {
    fn into_optional_rc(self) -> Option<Rc<T>> {
        self.map(|v| Rc::new(v))
    }
}

impl<T: ?Sized> IntoOptionalRc<T> for Option<Rc<T>> {
    fn into_optional_rc(self) -> Option<Rc<T>> {
        self
    }
}

impl<T> IntoOptionalRc<T> for T {
    fn into_optional_rc(self) -> Option<Rc<T>> {
        Some(Rc::new(self))
    }
}

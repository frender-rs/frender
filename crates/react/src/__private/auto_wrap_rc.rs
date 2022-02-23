pub mod __impl {
    use std::rc::Rc;

    pub trait CheckIsRc {
        type Output;
        fn check_is_rc(&self) -> Self::Output;
    }

    pub struct IsRc;

    impl IsRc {
        #[inline]
        pub fn auto_wrap_rc<T>(self, v: Rc<T>) -> Rc<T> {
            v
        }
    }

    pub struct NotRc;

    impl NotRc {
        #[inline]
        pub fn auto_wrap_rc<T>(self, v: T) -> Rc<T> {
            Rc::new(v)
        }
    }

    impl<T: ?Sized> CheckIsRc for Rc<T> {
        type Output = IsRc;

        #[inline]
        fn check_is_rc(&self) -> Self::Output {
            IsRc
        }
    }

    impl<T> CheckIsRc for &T {
        type Output = NotRc;

        fn check_is_rc(&self) -> Self::Output {
            NotRc
        }
    }
}

#[macro_export]
macro_rules! auto_wrap_rc {
    ($e:expr) => {{
        use $crate::__private::auto_wrap_rc::__impl::CheckIsRc;
        match $e {
            ref v => v.check_is_rc().auto_wrap_rc($e),
        }
    }};
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    #[test]
    fn auto_wrap_rc() {
        let a = crate::auto_wrap_rc!(1);
        let _: Rc<i32> = a;

        let b = crate::auto_wrap_rc!(Rc::new(true));
        let _: Rc<bool> = b;

        let c = String::new();
        let c = crate::auto_wrap_rc!(c);
        let _: Rc<String> = c;

        let d = Rc::new(String::new());
        let d = crate::auto_wrap_rc!(d);
        let _: Rc<String> = d;
    }

    #[test]
    fn nested() {
        let a = crate::auto_wrap_rc!(crate::auto_wrap_rc!(String::new()));
        let _: Rc<String> = a;
    }
}

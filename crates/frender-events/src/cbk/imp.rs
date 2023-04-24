// Recursive expansion of impl_with_macro_rules! macro
// ====================================================

use super::{Fn, Tuple};
impl Tuple for () {}

impl<Out, This: ?Sized> Fn<()> for This
where
    This: ::core::ops::Fn() -> Out,
{
    type Output = Out;
    fn call(&self, (): ()) -> Self::Output {
        self()
    }
}
impl<A1> Tuple for (A1,) {}

impl<A1, Out, This: ?Sized> Fn<(A1,)> for This
where
    This: ::core::ops::Fn(A1) -> Out,
{
    type Output = Out;
    fn call(&self, (a1,): (A1,)) -> Self::Output {
        self(a1)
    }
}
impl<A1, A2> Tuple for (A1, A2) {}

impl<A1, A2, Out, This: ?Sized> Fn<(A1, A2)> for This
where
    This: ::core::ops::Fn(A1, A2) -> Out,
{
    type Output = Out;
    fn call(&self, (a1, a2): (A1, A2)) -> Self::Output {
        self(a1, a2)
    }
}
impl<A1, A2, A3> Tuple for (A1, A2, A3) {}

impl<A1, A2, A3, Out, This: ?Sized> Fn<(A1, A2, A3)> for This
where
    This: ::core::ops::Fn(A1, A2, A3) -> Out,
{
    type Output = Out;
    fn call(&self, (a1, a2, a3): (A1, A2, A3)) -> Self::Output {
        self(a1, a2, a3)
    }
}
impl<A1, A2, A3, A4> Tuple for (A1, A2, A3, A4) {}

impl<A1, A2, A3, A4, Out, This: ?Sized> Fn<(A1, A2, A3, A4)> for This
where
    This: ::core::ops::Fn(A1, A2, A3, A4) -> Out,
{
    type Output = Out;
    fn call(&self, (a1, a2, a3, a4): (A1, A2, A3, A4)) -> Self::Output {
        self(a1, a2, a3, a4)
    }
}
impl<A1, A2, A3, A4, A5> Tuple for (A1, A2, A3, A4, A5) {}

impl<A1, A2, A3, A4, A5, Out, This: ?Sized> Fn<(A1, A2, A3, A4, A5)> for This
where
    This: ::core::ops::Fn(A1, A2, A3, A4, A5) -> Out,
{
    type Output = Out;
    fn call(&self, (a1, a2, a3, a4, a5): (A1, A2, A3, A4, A5)) -> Self::Output {
        self(a1, a2, a3, a4, a5)
    }
}
pub mod r#fn {
    #[derive(Debug, Clone, Copy)]
    pub struct HkFn<F>(F);

    pub fn value<A1, Out>(f: fn(A1) -> Out) -> fn(A1) -> Out {
        f
    }
    pub mod value {
        #[derive(Debug, Clone, Copy)]
        pub struct HkFn<F>(pub(super) F);

        pub fn value<A1, A2, Out>(f: fn(A1, A2) -> Out) -> fn(A1, A2) -> Out {
            f
        }
        pub mod value {
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);

            pub fn value<A1, A2, A3, Out>(f: fn(A1, A2, A3) -> Out) -> fn(A1, A2, A3) -> Out {
                f
            }
            pub mod value {
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);

                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(A1, A2, A3, A4) -> Out,
                ) -> fn(A1, A2, A3, A4) -> Out {
                    f
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, A3, A4, A5) -> Out,
                    ) -> fn(A1, A2, A3, A4, A5) -> Out {
                        f
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, A2, A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, A3, A4, &A5)>
                        for HkFn<fn(A1, A2, A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, A2, A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, A3, A4, &mut A5)>
                        for HkFn<fn(A1, A2, A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(A1, A2, A3, &A4) -> Out,
                ) -> HkFn<fn(A1, A2, A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, A2, A3, &A4)> for HkFn<fn(A1, A2, A3, &A4) -> Out> {
                    type Output = Out;
                    fn call(&self, args: (A1, A2, A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, A2, A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, A3, &A4, A5)>
                        for value::HkFn<fn(A1, A2, A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, A2, A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, A3, &A4, &A5)>
                        for HkFn<fn(A1, A2, A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, A2, A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, A3, &A4, &mut A5)>
                        for HkFn<fn(A1, A2, A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(A1, A2, A3, &mut A4) -> Out,
                ) -> HkFn<fn(A1, A2, A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, A2, A3, &mut A4)>
                    for HkFn<fn(A1, A2, A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, A2, A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, A2, A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, A3, &mut A4, A5)>
                        for value::HkFn<fn(A1, A2, A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, A2, A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, A3, &mut A4, &A5)>
                        for HkFn<fn(A1, A2, A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, A2, A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, A3, &mut A4, &mut A5)>
                        for HkFn<fn(A1, A2, A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#ref<A1, A2, A3, Out>(
                f: fn(A1, A2, &A3) -> Out,
            ) -> HkFn<fn(A1, A2, &A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(A1, A2, &A3)> for HkFn<fn(A1, A2, &A3) -> Out> {
                type Output = Out;
                fn call(&self, args: (A1, A2, &A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#ref {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(A1, A2, &A3, A4) -> Out,
                ) -> value::HkFn<fn(A1, A2, &A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, A2, &A3, A4)>
                    for value::HkFn<fn(A1, A2, &A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, A2, &A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, A2, &A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &A3, A4, A5)>
                        for value::HkFn<fn(A1, A2, &A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, A2, &A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &A3, A4, &A5)>
                        for HkFn<fn(A1, A2, &A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, A2, &A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &A3, A4, &mut A5)>
                        for HkFn<fn(A1, A2, &A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(A1, A2, &A3, &A4) -> Out,
                ) -> HkFn<fn(A1, A2, &A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, A2, &A3, &A4)> for HkFn<fn(A1, A2, &A3, &A4) -> Out> {
                    type Output = Out;
                    fn call(&self, args: (A1, A2, &A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, A2, &A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &A3, &A4, A5)>
                        for value::HkFn<fn(A1, A2, &A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, A2, &A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &A3, &A4, &A5)>
                        for HkFn<fn(A1, A2, &A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, A2, &A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &A3, &A4, &mut A5)>
                        for HkFn<fn(A1, A2, &A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(A1, A2, &A3, &mut A4) -> Out,
                ) -> HkFn<fn(A1, A2, &A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, A2, &A3, &mut A4)>
                    for HkFn<fn(A1, A2, &A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, A2, &A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, A2, &A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &A3, &mut A4, A5)>
                        for value::HkFn<fn(A1, A2, &A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, A2, &A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &A3, &mut A4, &A5)>
                        for HkFn<fn(A1, A2, &A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, A2, &A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &A3, &mut A4, &mut A5)>
                        for HkFn<fn(A1, A2, &A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#mut<A1, A2, A3, Out>(
                f: fn(A1, A2, &mut A3) -> Out,
            ) -> HkFn<fn(A1, A2, &mut A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(A1, A2, &mut A3)> for HkFn<fn(A1, A2, &mut A3) -> Out> {
                type Output = Out;
                fn call(&self, args: (A1, A2, &mut A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#mut {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(A1, A2, &mut A3, A4) -> Out,
                ) -> value::HkFn<fn(A1, A2, &mut A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, A2, &mut A3, A4)>
                    for value::HkFn<fn(A1, A2, &mut A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, A2, &mut A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &mut A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, A2, &mut A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &mut A3, A4, A5)>
                        for value::HkFn<fn(A1, A2, &mut A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &mut A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &mut A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, A2, &mut A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &mut A3, A4, &A5)>
                        for HkFn<fn(A1, A2, &mut A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &mut A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &mut A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, A2, &mut A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &mut A3, A4, &mut A5)>
                        for HkFn<fn(A1, A2, &mut A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &mut A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(A1, A2, &mut A3, &A4) -> Out,
                ) -> HkFn<fn(A1, A2, &mut A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, A2, &mut A3, &A4)>
                    for HkFn<fn(A1, A2, &mut A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, A2, &mut A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &mut A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, A2, &mut A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &mut A3, &A4, A5)>
                        for value::HkFn<fn(A1, A2, &mut A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &mut A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &mut A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, A2, &mut A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &mut A3, &A4, &A5)>
                        for HkFn<fn(A1, A2, &mut A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &mut A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &mut A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, A2, &mut A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &mut A3, &A4, &mut A5)>
                        for HkFn<fn(A1, A2, &mut A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &mut A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(A1, A2, &mut A3, &mut A4) -> Out,
                ) -> HkFn<fn(A1, A2, &mut A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, A2, &mut A3, &mut A4)>
                    for HkFn<fn(A1, A2, &mut A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, A2, &mut A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &mut A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, A2, &mut A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &mut A3, &mut A4, A5)>
                        for value::HkFn<fn(A1, A2, &mut A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &mut A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &mut A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, A2, &mut A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, A2, &mut A3, &mut A4, &A5)>
                        for HkFn<fn(A1, A2, &mut A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &mut A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, A2, &mut A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, A2, &mut A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(A1, A2, &mut A3, &mut A4, &mut A5)>
                        for HkFn<fn(A1, A2, &mut A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, A2, &mut A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
        }
        pub fn r#ref<A1, A2, Out>(f: fn(A1, &A2) -> Out) -> HkFn<fn(A1, &A2) -> Out> {
            HkFn(f)
        }
        impl<A1, A2, Out> crate::cbk::Fn<(A1, &A2)> for HkFn<fn(A1, &A2) -> Out> {
            type Output = Out;
            fn call(&self, args: (A1, &A2)) -> Self::Output {
                crate::cbk::Fn::<_>::call(&self.0, args)
            }
        }
        pub mod r#ref {
            use super::HkFn;
            pub fn value<A1, A2, A3, Out>(
                f: fn(A1, &A2, A3) -> Out,
            ) -> value::HkFn<fn(A1, &A2, A3) -> Out> {
                value::HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(A1, &A2, A3)> for value::HkFn<fn(A1, &A2, A3) -> Out> {
                type Output = Out;
                fn call(&self, args: (A1, &A2, A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod value {
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);

                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(A1, &A2, A3, A4) -> Out,
                ) -> value::HkFn<fn(A1, &A2, A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &A2, A3, A4)>
                    for value::HkFn<fn(A1, &A2, A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &A2, A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &A2, A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, A3, A4, A5)>
                        for value::HkFn<fn(A1, &A2, A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, A3, A4, &A5)>
                        for HkFn<fn(A1, &A2, A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, A3, A4, &mut A5)>
                        for HkFn<fn(A1, &A2, A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(A1, &A2, A3, &A4) -> Out,
                ) -> HkFn<fn(A1, &A2, A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &A2, A3, &A4)> for HkFn<fn(A1, &A2, A3, &A4) -> Out> {
                    type Output = Out;
                    fn call(&self, args: (A1, &A2, A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &A2, A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, A3, &A4, A5)>
                        for value::HkFn<fn(A1, &A2, A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, A3, &A4, &A5)>
                        for HkFn<fn(A1, &A2, A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, A3, &A4, &mut A5)>
                        for HkFn<fn(A1, &A2, A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(A1, &A2, A3, &mut A4) -> Out,
                ) -> HkFn<fn(A1, &A2, A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &A2, A3, &mut A4)>
                    for HkFn<fn(A1, &A2, A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &A2, A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &A2, A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, A3, &mut A4, A5)>
                        for value::HkFn<fn(A1, &A2, A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, A3, &mut A4, &A5)>
                        for HkFn<fn(A1, &A2, A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, A3, &mut A4, &mut A5)>
                        for HkFn<fn(A1, &A2, A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#ref<A1, A2, A3, Out>(
                f: fn(A1, &A2, &A3) -> Out,
            ) -> HkFn<fn(A1, &A2, &A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(A1, &A2, &A3)> for HkFn<fn(A1, &A2, &A3) -> Out> {
                type Output = Out;
                fn call(&self, args: (A1, &A2, &A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#ref {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(A1, &A2, &A3, A4) -> Out,
                ) -> value::HkFn<fn(A1, &A2, &A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &A2, &A3, A4)>
                    for value::HkFn<fn(A1, &A2, &A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &A2, &A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &A2, &A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &A3, A4, A5)>
                        for value::HkFn<fn(A1, &A2, &A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, &A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &A3, A4, &A5)>
                        for HkFn<fn(A1, &A2, &A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, &A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &A3, A4, &mut A5)>
                        for HkFn<fn(A1, &A2, &A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(A1, &A2, &A3, &A4) -> Out,
                ) -> HkFn<fn(A1, &A2, &A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &A2, &A3, &A4)>
                    for HkFn<fn(A1, &A2, &A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &A2, &A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &A2, &A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &A3, &A4, A5)>
                        for value::HkFn<fn(A1, &A2, &A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, &A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &A3, &A4, &A5)>
                        for HkFn<fn(A1, &A2, &A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, &A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &A3, &A4, &mut A5)>
                        for HkFn<fn(A1, &A2, &A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(A1, &A2, &A3, &mut A4) -> Out,
                ) -> HkFn<fn(A1, &A2, &A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &A2, &A3, &mut A4)>
                    for HkFn<fn(A1, &A2, &A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &A2, &A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &A2, &A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &A3, &mut A4, A5)>
                        for value::HkFn<fn(A1, &A2, &A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, &A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &A3, &mut A4, &A5)>
                        for HkFn<fn(A1, &A2, &A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, &A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &A3, &mut A4, &mut A5)>
                        for HkFn<fn(A1, &A2, &A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#mut<A1, A2, A3, Out>(
                f: fn(A1, &A2, &mut A3) -> Out,
            ) -> HkFn<fn(A1, &A2, &mut A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(A1, &A2, &mut A3)> for HkFn<fn(A1, &A2, &mut A3) -> Out> {
                type Output = Out;
                fn call(&self, args: (A1, &A2, &mut A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#mut {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(A1, &A2, &mut A3, A4) -> Out,
                ) -> value::HkFn<fn(A1, &A2, &mut A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &A2, &mut A3, A4)>
                    for value::HkFn<fn(A1, &A2, &mut A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &A2, &mut A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &mut A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &A2, &mut A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &mut A3, A4, A5)>
                        for value::HkFn<fn(A1, &A2, &mut A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &mut A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &mut A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, &mut A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &mut A3, A4, &A5)>
                        for HkFn<fn(A1, &A2, &mut A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &mut A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &mut A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, &mut A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &mut A3, A4, &mut A5)>
                        for HkFn<fn(A1, &A2, &mut A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &mut A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(A1, &A2, &mut A3, &A4) -> Out,
                ) -> HkFn<fn(A1, &A2, &mut A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &A2, &mut A3, &A4)>
                    for HkFn<fn(A1, &A2, &mut A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &A2, &mut A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &mut A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &A2, &mut A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &mut A3, &A4, A5)>
                        for value::HkFn<fn(A1, &A2, &mut A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &mut A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &mut A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, &mut A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &mut A3, &A4, &A5)>
                        for HkFn<fn(A1, &A2, &mut A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &mut A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &mut A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, &mut A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &mut A3, &A4, &mut A5)>
                        for HkFn<fn(A1, &A2, &mut A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &mut A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(A1, &A2, &mut A3, &mut A4) -> Out,
                ) -> HkFn<fn(A1, &A2, &mut A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &A2, &mut A3, &mut A4)>
                    for HkFn<fn(A1, &A2, &mut A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &A2, &mut A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &mut A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &A2, &mut A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &mut A3, &mut A4, A5)>
                        for value::HkFn<fn(A1, &A2, &mut A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &mut A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &mut A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, &mut A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &A2, &mut A3, &mut A4, &A5)>
                        for HkFn<fn(A1, &A2, &mut A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &mut A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &A2, &mut A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &A2, &mut A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(A1, &A2, &mut A3, &mut A4, &mut A5)>
                        for HkFn<fn(A1, &A2, &mut A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &A2, &mut A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
        }
        pub fn r#mut<A1, A2, Out>(f: fn(A1, &mut A2) -> Out) -> HkFn<fn(A1, &mut A2) -> Out> {
            HkFn(f)
        }
        impl<A1, A2, Out> crate::cbk::Fn<(A1, &mut A2)> for HkFn<fn(A1, &mut A2) -> Out> {
            type Output = Out;
            fn call(&self, args: (A1, &mut A2)) -> Self::Output {
                crate::cbk::Fn::<_>::call(&self.0, args)
            }
        }
        pub mod r#mut {
            use super::HkFn;
            pub fn value<A1, A2, A3, Out>(
                f: fn(A1, &mut A2, A3) -> Out,
            ) -> value::HkFn<fn(A1, &mut A2, A3) -> Out> {
                value::HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(A1, &mut A2, A3)>
                for value::HkFn<fn(A1, &mut A2, A3) -> Out>
            {
                type Output = Out;
                fn call(&self, args: (A1, &mut A2, A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod value {
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);

                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(A1, &mut A2, A3, A4) -> Out,
                ) -> value::HkFn<fn(A1, &mut A2, A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &mut A2, A3, A4)>
                    for value::HkFn<fn(A1, &mut A2, A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &mut A2, A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &mut A2, A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, A3, A4, A5)>
                        for value::HkFn<fn(A1, &mut A2, A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, A3, A4, &A5)>
                        for HkFn<fn(A1, &mut A2, A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, A3, A4, &mut A5)>
                        for HkFn<fn(A1, &mut A2, A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(A1, &mut A2, A3, &A4) -> Out,
                ) -> HkFn<fn(A1, &mut A2, A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &mut A2, A3, &A4)>
                    for HkFn<fn(A1, &mut A2, A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &mut A2, A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &mut A2, A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, A3, &A4, A5)>
                        for value::HkFn<fn(A1, &mut A2, A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, A3, &A4, &A5)>
                        for HkFn<fn(A1, &mut A2, A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, A3, &A4, &mut A5)>
                        for HkFn<fn(A1, &mut A2, A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(A1, &mut A2, A3, &mut A4) -> Out,
                ) -> HkFn<fn(A1, &mut A2, A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &mut A2, A3, &mut A4)>
                    for HkFn<fn(A1, &mut A2, A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &mut A2, A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &mut A2, A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, A3, &mut A4, A5)>
                        for value::HkFn<fn(A1, &mut A2, A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, A3, &mut A4, &A5)>
                        for HkFn<fn(A1, &mut A2, A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(A1, &mut A2, A3, &mut A4, &mut A5)>
                        for HkFn<fn(A1, &mut A2, A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#ref<A1, A2, A3, Out>(
                f: fn(A1, &mut A2, &A3) -> Out,
            ) -> HkFn<fn(A1, &mut A2, &A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(A1, &mut A2, &A3)> for HkFn<fn(A1, &mut A2, &A3) -> Out> {
                type Output = Out;
                fn call(&self, args: (A1, &mut A2, &A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#ref {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(A1, &mut A2, &A3, A4) -> Out,
                ) -> value::HkFn<fn(A1, &mut A2, &A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &mut A2, &A3, A4)>
                    for value::HkFn<fn(A1, &mut A2, &A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &mut A2, &A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &mut A2, &A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, &A3, A4, A5)>
                        for value::HkFn<fn(A1, &mut A2, &A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, &A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, &A3, A4, &A5)>
                        for HkFn<fn(A1, &mut A2, &A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, &A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, &A3, A4, &mut A5)>
                        for HkFn<fn(A1, &mut A2, &A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(A1, &mut A2, &A3, &A4) -> Out,
                ) -> HkFn<fn(A1, &mut A2, &A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &mut A2, &A3, &A4)>
                    for HkFn<fn(A1, &mut A2, &A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &mut A2, &A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &mut A2, &A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, &A3, &A4, A5)>
                        for value::HkFn<fn(A1, &mut A2, &A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, &A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, &A3, &A4, &A5)>
                        for HkFn<fn(A1, &mut A2, &A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, &A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, &A3, &A4, &mut A5)>
                        for HkFn<fn(A1, &mut A2, &A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(A1, &mut A2, &A3, &mut A4) -> Out,
                ) -> HkFn<fn(A1, &mut A2, &A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &mut A2, &A3, &mut A4)>
                    for HkFn<fn(A1, &mut A2, &A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &mut A2, &A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &mut A2, &A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, &A3, &mut A4, A5)>
                        for value::HkFn<fn(A1, &mut A2, &A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, &A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, &A3, &mut A4, &A5)>
                        for HkFn<fn(A1, &mut A2, &A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, &A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(A1, &mut A2, &A3, &mut A4, &mut A5)>
                        for HkFn<fn(A1, &mut A2, &A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#mut<A1, A2, A3, Out>(
                f: fn(A1, &mut A2, &mut A3) -> Out,
            ) -> HkFn<fn(A1, &mut A2, &mut A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(A1, &mut A2, &mut A3)>
                for HkFn<fn(A1, &mut A2, &mut A3) -> Out>
            {
                type Output = Out;
                fn call(&self, args: (A1, &mut A2, &mut A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#mut {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(A1, &mut A2, &mut A3, A4) -> Out,
                ) -> value::HkFn<fn(A1, &mut A2, &mut A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &mut A2, &mut A3, A4)>
                    for value::HkFn<fn(A1, &mut A2, &mut A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &mut A2, &mut A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &mut A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &mut A2, &mut A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, &mut A3, A4, A5)>
                        for value::HkFn<fn(A1, &mut A2, &mut A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &mut A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &mut A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, &mut A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, &mut A3, A4, &A5)>
                        for HkFn<fn(A1, &mut A2, &mut A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &mut A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &mut A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, &mut A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(A1, &mut A2, &mut A3, A4, &mut A5)>
                        for HkFn<fn(A1, &mut A2, &mut A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &mut A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(A1, &mut A2, &mut A3, &A4) -> Out,
                ) -> HkFn<fn(A1, &mut A2, &mut A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &mut A2, &mut A3, &A4)>
                    for HkFn<fn(A1, &mut A2, &mut A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &mut A2, &mut A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &mut A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &mut A2, &mut A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, &mut A3, &A4, A5)>
                        for value::HkFn<fn(A1, &mut A2, &mut A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &mut A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &mut A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, &mut A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(A1, &mut A2, &mut A3, &A4, &A5)>
                        for HkFn<fn(A1, &mut A2, &mut A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &mut A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &mut A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, &mut A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(A1, &mut A2, &mut A3, &A4, &mut A5)>
                        for HkFn<fn(A1, &mut A2, &mut A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &mut A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(A1, &mut A2, &mut A3, &mut A4) -> Out,
                ) -> HkFn<fn(A1, &mut A2, &mut A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(A1, &mut A2, &mut A3, &mut A4)>
                    for HkFn<fn(A1, &mut A2, &mut A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (A1, &mut A2, &mut A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &mut A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(A1, &mut A2, &mut A3, &mut A4, A5) -> Out>
                    {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(A1, &mut A2, &mut A3, &mut A4, A5)>
                        for value::HkFn<fn(A1, &mut A2, &mut A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &mut A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &mut A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, &mut A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(A1, &mut A2, &mut A3, &mut A4, &A5)>
                        for HkFn<fn(A1, &mut A2, &mut A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (A1, &mut A2, &mut A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(A1, &mut A2, &mut A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, &mut A3, &mut A4, &mut A5) -> Out>
                    {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(A1, &mut A2, &mut A3, &mut A4, &mut A5)>
                        for HkFn<fn(A1, &mut A2, &mut A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (A1, &mut A2, &mut A3, &mut A4, &mut A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
        }
    }
    pub fn r#ref<A1, Out>(f: fn(&A1) -> Out) -> HkFn<fn(&A1) -> Out> {
        HkFn(f)
    }
    impl<A1, Out> crate::cbk::Fn<(&A1,)> for HkFn<fn(&A1) -> Out> {
        type Output = Out;
        fn call(&self, args: (&A1,)) -> Self::Output {
            crate::cbk::Fn::<_>::call(&self.0, args)
        }
    }
    pub mod r#ref {
        use super::HkFn;
        pub fn value<A1, A2, Out>(f: fn(&A1, A2) -> Out) -> value::HkFn<fn(&A1, A2) -> Out> {
            value::HkFn(f)
        }
        impl<A1, A2, Out> crate::cbk::Fn<(&A1, A2)> for value::HkFn<fn(&A1, A2) -> Out> {
            type Output = Out;
            fn call(&self, args: (&A1, A2)) -> Self::Output {
                crate::cbk::Fn::<_>::call(&self.0, args)
            }
        }
        pub mod value {
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);

            pub fn value<A1, A2, A3, Out>(
                f: fn(&A1, A2, A3) -> Out,
            ) -> value::HkFn<fn(&A1, A2, A3) -> Out> {
                value::HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&A1, A2, A3)> for value::HkFn<fn(&A1, A2, A3) -> Out> {
                type Output = Out;
                fn call(&self, args: (&A1, A2, A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod value {
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);

                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&A1, A2, A3, A4) -> Out,
                ) -> value::HkFn<fn(&A1, A2, A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, A2, A3, A4)>
                    for value::HkFn<fn(&A1, A2, A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, A2, A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, A2, A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, A3, A4, A5)>
                        for value::HkFn<fn(&A1, A2, A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, A3, A4, &A5)>
                        for HkFn<fn(&A1, A2, A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, A3, A4, &mut A5)>
                        for HkFn<fn(&A1, A2, A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&A1, A2, A3, &A4) -> Out,
                ) -> HkFn<fn(&A1, A2, A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, A2, A3, &A4)> for HkFn<fn(&A1, A2, A3, &A4) -> Out> {
                    type Output = Out;
                    fn call(&self, args: (&A1, A2, A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, A2, A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, A3, &A4, A5)>
                        for value::HkFn<fn(&A1, A2, A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, A3, &A4, &A5)>
                        for HkFn<fn(&A1, A2, A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, A3, &A4, &mut A5)>
                        for HkFn<fn(&A1, A2, A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&A1, A2, A3, &mut A4) -> Out,
                ) -> HkFn<fn(&A1, A2, A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, A2, A3, &mut A4)>
                    for HkFn<fn(&A1, A2, A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, A2, A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, A2, A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, A3, &mut A4, A5)>
                        for value::HkFn<fn(&A1, A2, A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, A3, &mut A4, &A5)>
                        for HkFn<fn(&A1, A2, A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, A3, &mut A4, &mut A5)>
                        for HkFn<fn(&A1, A2, A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#ref<A1, A2, A3, Out>(
                f: fn(&A1, A2, &A3) -> Out,
            ) -> HkFn<fn(&A1, A2, &A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&A1, A2, &A3)> for HkFn<fn(&A1, A2, &A3) -> Out> {
                type Output = Out;
                fn call(&self, args: (&A1, A2, &A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#ref {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&A1, A2, &A3, A4) -> Out,
                ) -> value::HkFn<fn(&A1, A2, &A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, A2, &A3, A4)>
                    for value::HkFn<fn(&A1, A2, &A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, A2, &A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, A2, &A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &A3, A4, A5)>
                        for value::HkFn<fn(&A1, A2, &A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, &A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &A3, A4, &A5)>
                        for HkFn<fn(&A1, A2, &A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, &A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &A3, A4, &mut A5)>
                        for HkFn<fn(&A1, A2, &A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&A1, A2, &A3, &A4) -> Out,
                ) -> HkFn<fn(&A1, A2, &A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, A2, &A3, &A4)>
                    for HkFn<fn(&A1, A2, &A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, A2, &A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, A2, &A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &A3, &A4, A5)>
                        for value::HkFn<fn(&A1, A2, &A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, &A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &A3, &A4, &A5)>
                        for HkFn<fn(&A1, A2, &A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, &A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &A3, &A4, &mut A5)>
                        for HkFn<fn(&A1, A2, &A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&A1, A2, &A3, &mut A4) -> Out,
                ) -> HkFn<fn(&A1, A2, &A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, A2, &A3, &mut A4)>
                    for HkFn<fn(&A1, A2, &A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, A2, &A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, A2, &A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &A3, &mut A4, A5)>
                        for value::HkFn<fn(&A1, A2, &A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, &A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &A3, &mut A4, &A5)>
                        for HkFn<fn(&A1, A2, &A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, &A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &A3, &mut A4, &mut A5)>
                        for HkFn<fn(&A1, A2, &A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#mut<A1, A2, A3, Out>(
                f: fn(&A1, A2, &mut A3) -> Out,
            ) -> HkFn<fn(&A1, A2, &mut A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&A1, A2, &mut A3)> for HkFn<fn(&A1, A2, &mut A3) -> Out> {
                type Output = Out;
                fn call(&self, args: (&A1, A2, &mut A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#mut {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&A1, A2, &mut A3, A4) -> Out,
                ) -> value::HkFn<fn(&A1, A2, &mut A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, A2, &mut A3, A4)>
                    for value::HkFn<fn(&A1, A2, &mut A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, A2, &mut A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &mut A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, A2, &mut A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &mut A3, A4, A5)>
                        for value::HkFn<fn(&A1, A2, &mut A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &mut A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &mut A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, &mut A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &mut A3, A4, &A5)>
                        for HkFn<fn(&A1, A2, &mut A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &mut A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &mut A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, &mut A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &mut A3, A4, &mut A5)>
                        for HkFn<fn(&A1, A2, &mut A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &mut A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&A1, A2, &mut A3, &A4) -> Out,
                ) -> HkFn<fn(&A1, A2, &mut A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, A2, &mut A3, &A4)>
                    for HkFn<fn(&A1, A2, &mut A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, A2, &mut A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &mut A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, A2, &mut A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &mut A3, &A4, A5)>
                        for value::HkFn<fn(&A1, A2, &mut A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &mut A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &mut A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, &mut A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &mut A3, &A4, &A5)>
                        for HkFn<fn(&A1, A2, &mut A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &mut A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &mut A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, &mut A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &mut A3, &A4, &mut A5)>
                        for HkFn<fn(&A1, A2, &mut A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &mut A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&A1, A2, &mut A3, &mut A4) -> Out,
                ) -> HkFn<fn(&A1, A2, &mut A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, A2, &mut A3, &mut A4)>
                    for HkFn<fn(&A1, A2, &mut A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, A2, &mut A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &mut A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, A2, &mut A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &mut A3, &mut A4, A5)>
                        for value::HkFn<fn(&A1, A2, &mut A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &mut A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &mut A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, &mut A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, A2, &mut A3, &mut A4, &A5)>
                        for HkFn<fn(&A1, A2, &mut A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &mut A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, A2, &mut A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, A2, &mut A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&A1, A2, &mut A3, &mut A4, &mut A5)>
                        for HkFn<fn(&A1, A2, &mut A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, A2, &mut A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
        }
        pub fn r#ref<A1, A2, Out>(f: fn(&A1, &A2) -> Out) -> HkFn<fn(&A1, &A2) -> Out> {
            HkFn(f)
        }
        impl<A1, A2, Out> crate::cbk::Fn<(&A1, &A2)> for HkFn<fn(&A1, &A2) -> Out> {
            type Output = Out;
            fn call(&self, args: (&A1, &A2)) -> Self::Output {
                crate::cbk::Fn::<_>::call(&self.0, args)
            }
        }
        pub mod r#ref {
            use super::HkFn;
            pub fn value<A1, A2, A3, Out>(
                f: fn(&A1, &A2, A3) -> Out,
            ) -> value::HkFn<fn(&A1, &A2, A3) -> Out> {
                value::HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&A1, &A2, A3)> for value::HkFn<fn(&A1, &A2, A3) -> Out> {
                type Output = Out;
                fn call(&self, args: (&A1, &A2, A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod value {
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);

                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &A2, A3, A4) -> Out,
                ) -> value::HkFn<fn(&A1, &A2, A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &A2, A3, A4)>
                    for value::HkFn<fn(&A1, &A2, A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &A2, A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &A2, A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, A3, A4, A5)>
                        for value::HkFn<fn(&A1, &A2, A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, A3, A4, &A5)>
                        for HkFn<fn(&A1, &A2, A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, A3, A4, &mut A5)>
                        for HkFn<fn(&A1, &A2, A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &A2, A3, &A4) -> Out,
                ) -> HkFn<fn(&A1, &A2, A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &A2, A3, &A4)>
                    for HkFn<fn(&A1, &A2, A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &A2, A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &A2, A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, A3, &A4, A5)>
                        for value::HkFn<fn(&A1, &A2, A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, A3, &A4, &A5)>
                        for HkFn<fn(&A1, &A2, A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, A3, &A4, &mut A5)>
                        for HkFn<fn(&A1, &A2, A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &A2, A3, &mut A4) -> Out,
                ) -> HkFn<fn(&A1, &A2, A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &A2, A3, &mut A4)>
                    for HkFn<fn(&A1, &A2, A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &A2, A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &A2, A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, A3, &mut A4, A5)>
                        for value::HkFn<fn(&A1, &A2, A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, A3, &mut A4, &A5)>
                        for HkFn<fn(&A1, &A2, A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, A3, &mut A4, &mut A5)>
                        for HkFn<fn(&A1, &A2, A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#ref<A1, A2, A3, Out>(
                f: fn(&A1, &A2, &A3) -> Out,
            ) -> HkFn<fn(&A1, &A2, &A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&A1, &A2, &A3)> for HkFn<fn(&A1, &A2, &A3) -> Out> {
                type Output = Out;
                fn call(&self, args: (&A1, &A2, &A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#ref {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &A2, &A3, A4) -> Out,
                ) -> value::HkFn<fn(&A1, &A2, &A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &A2, &A3, A4)>
                    for value::HkFn<fn(&A1, &A2, &A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &A2, &A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &A2, &A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &A3, A4, A5)>
                        for value::HkFn<fn(&A1, &A2, &A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, &A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &A3, A4, &A5)>
                        for HkFn<fn(&A1, &A2, &A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, &A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &A3, A4, &mut A5)>
                        for HkFn<fn(&A1, &A2, &A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &A2, &A3, &A4) -> Out,
                ) -> HkFn<fn(&A1, &A2, &A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &A2, &A3, &A4)>
                    for HkFn<fn(&A1, &A2, &A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &A2, &A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &A2, &A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &A3, &A4, A5)>
                        for value::HkFn<fn(&A1, &A2, &A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, &A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &A3, &A4, &A5)>
                        for HkFn<fn(&A1, &A2, &A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, &A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &A3, &A4, &mut A5)>
                        for HkFn<fn(&A1, &A2, &A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &A2, &A3, &mut A4) -> Out,
                ) -> HkFn<fn(&A1, &A2, &A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &A2, &A3, &mut A4)>
                    for HkFn<fn(&A1, &A2, &A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &A2, &A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &A2, &A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &A3, &mut A4, A5)>
                        for value::HkFn<fn(&A1, &A2, &A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, &A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &A3, &mut A4, &A5)>
                        for HkFn<fn(&A1, &A2, &A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, &A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &A3, &mut A4, &mut A5)>
                        for HkFn<fn(&A1, &A2, &A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#mut<A1, A2, A3, Out>(
                f: fn(&A1, &A2, &mut A3) -> Out,
            ) -> HkFn<fn(&A1, &A2, &mut A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&A1, &A2, &mut A3)> for HkFn<fn(&A1, &A2, &mut A3) -> Out> {
                type Output = Out;
                fn call(&self, args: (&A1, &A2, &mut A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#mut {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &A2, &mut A3, A4) -> Out,
                ) -> value::HkFn<fn(&A1, &A2, &mut A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &A2, &mut A3, A4)>
                    for value::HkFn<fn(&A1, &A2, &mut A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &A2, &mut A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &mut A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &A2, &mut A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &mut A3, A4, A5)>
                        for value::HkFn<fn(&A1, &A2, &mut A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &mut A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &mut A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, &mut A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &mut A3, A4, &A5)>
                        for HkFn<fn(&A1, &A2, &mut A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &mut A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &mut A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, &mut A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &mut A3, A4, &mut A5)>
                        for HkFn<fn(&A1, &A2, &mut A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &mut A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &A2, &mut A3, &A4) -> Out,
                ) -> HkFn<fn(&A1, &A2, &mut A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &A2, &mut A3, &A4)>
                    for HkFn<fn(&A1, &A2, &mut A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &A2, &mut A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &mut A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &A2, &mut A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &mut A3, &A4, A5)>
                        for value::HkFn<fn(&A1, &A2, &mut A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &mut A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &mut A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, &mut A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &mut A3, &A4, &A5)>
                        for HkFn<fn(&A1, &A2, &mut A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &mut A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &mut A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, &mut A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &mut A3, &A4, &mut A5)>
                        for HkFn<fn(&A1, &A2, &mut A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &mut A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &A2, &mut A3, &mut A4) -> Out,
                ) -> HkFn<fn(&A1, &A2, &mut A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &A2, &mut A3, &mut A4)>
                    for HkFn<fn(&A1, &A2, &mut A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &A2, &mut A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &mut A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &A2, &mut A3, &mut A4, A5) -> Out>
                    {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &mut A3, &mut A4, A5)>
                        for value::HkFn<fn(&A1, &A2, &mut A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &mut A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &mut A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, &mut A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &A2, &mut A3, &mut A4, &A5)>
                        for HkFn<fn(&A1, &A2, &mut A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &A2, &mut A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &A2, &mut A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &A2, &mut A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&A1, &A2, &mut A3, &mut A4, &mut A5)>
                        for HkFn<fn(&A1, &A2, &mut A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&A1, &A2, &mut A3, &mut A4, &mut A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
        }
        pub fn r#mut<A1, A2, Out>(f: fn(&A1, &mut A2) -> Out) -> HkFn<fn(&A1, &mut A2) -> Out> {
            HkFn(f)
        }
        impl<A1, A2, Out> crate::cbk::Fn<(&A1, &mut A2)> for HkFn<fn(&A1, &mut A2) -> Out> {
            type Output = Out;
            fn call(&self, args: (&A1, &mut A2)) -> Self::Output {
                crate::cbk::Fn::<_>::call(&self.0, args)
            }
        }
        pub mod r#mut {
            use super::HkFn;
            pub fn value<A1, A2, A3, Out>(
                f: fn(&A1, &mut A2, A3) -> Out,
            ) -> value::HkFn<fn(&A1, &mut A2, A3) -> Out> {
                value::HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&A1, &mut A2, A3)>
                for value::HkFn<fn(&A1, &mut A2, A3) -> Out>
            {
                type Output = Out;
                fn call(&self, args: (&A1, &mut A2, A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod value {
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);

                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &mut A2, A3, A4) -> Out,
                ) -> value::HkFn<fn(&A1, &mut A2, A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &mut A2, A3, A4)>
                    for value::HkFn<fn(&A1, &mut A2, A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &mut A2, A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &mut A2, A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, A3, A4, A5)>
                        for value::HkFn<fn(&A1, &mut A2, A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, A3, A4, &A5)>
                        for HkFn<fn(&A1, &mut A2, A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, A3, A4, &mut A5)>
                        for HkFn<fn(&A1, &mut A2, A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &mut A2, A3, &A4) -> Out,
                ) -> HkFn<fn(&A1, &mut A2, A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &mut A2, A3, &A4)>
                    for HkFn<fn(&A1, &mut A2, A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &mut A2, A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &mut A2, A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, A3, &A4, A5)>
                        for value::HkFn<fn(&A1, &mut A2, A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, A3, &A4, &A5)>
                        for HkFn<fn(&A1, &mut A2, A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, A3, &A4, &mut A5)>
                        for HkFn<fn(&A1, &mut A2, A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &mut A2, A3, &mut A4) -> Out,
                ) -> HkFn<fn(&A1, &mut A2, A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &mut A2, A3, &mut A4)>
                    for HkFn<fn(&A1, &mut A2, A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &mut A2, A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &mut A2, A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, A3, &mut A4, A5)>
                        for value::HkFn<fn(&A1, &mut A2, A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, A3, &mut A4, &A5)>
                        for HkFn<fn(&A1, &mut A2, A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&A1, &mut A2, A3, &mut A4, &mut A5)>
                        for HkFn<fn(&A1, &mut A2, A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#ref<A1, A2, A3, Out>(
                f: fn(&A1, &mut A2, &A3) -> Out,
            ) -> HkFn<fn(&A1, &mut A2, &A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&A1, &mut A2, &A3)> for HkFn<fn(&A1, &mut A2, &A3) -> Out> {
                type Output = Out;
                fn call(&self, args: (&A1, &mut A2, &A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#ref {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &mut A2, &A3, A4) -> Out,
                ) -> value::HkFn<fn(&A1, &mut A2, &A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &mut A2, &A3, A4)>
                    for value::HkFn<fn(&A1, &mut A2, &A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &mut A2, &A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &mut A2, &A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, &A3, A4, A5)>
                        for value::HkFn<fn(&A1, &mut A2, &A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, &A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, &A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, &A3, A4, &A5)>
                        for HkFn<fn(&A1, &mut A2, &A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, &A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, &A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, &A3, A4, &mut A5)>
                        for HkFn<fn(&A1, &mut A2, &A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, &A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &mut A2, &A3, &A4) -> Out,
                ) -> HkFn<fn(&A1, &mut A2, &A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &mut A2, &A3, &A4)>
                    for HkFn<fn(&A1, &mut A2, &A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &mut A2, &A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &mut A2, &A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, &A3, &A4, A5)>
                        for value::HkFn<fn(&A1, &mut A2, &A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, &A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, &A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, &A3, &A4, &A5)>
                        for HkFn<fn(&A1, &mut A2, &A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, &A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, &A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, &A3, &A4, &mut A5)>
                        for HkFn<fn(&A1, &mut A2, &A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, &A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &mut A2, &A3, &mut A4) -> Out,
                ) -> HkFn<fn(&A1, &mut A2, &A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &mut A2, &A3, &mut A4)>
                    for HkFn<fn(&A1, &mut A2, &A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &mut A2, &A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &mut A2, &A3, &mut A4, A5) -> Out>
                    {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, &A3, &mut A4, A5)>
                        for value::HkFn<fn(&A1, &mut A2, &A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, &A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, &A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, &A3, &mut A4, &A5)>
                        for HkFn<fn(&A1, &mut A2, &A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, &A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, &A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&A1, &mut A2, &A3, &mut A4, &mut A5)>
                        for HkFn<fn(&A1, &mut A2, &A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&A1, &mut A2, &A3, &mut A4, &mut A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#mut<A1, A2, A3, Out>(
                f: fn(&A1, &mut A2, &mut A3) -> Out,
            ) -> HkFn<fn(&A1, &mut A2, &mut A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&A1, &mut A2, &mut A3)>
                for HkFn<fn(&A1, &mut A2, &mut A3) -> Out>
            {
                type Output = Out;
                fn call(&self, args: (&A1, &mut A2, &mut A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#mut {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &mut A2, &mut A3, A4) -> Out,
                ) -> value::HkFn<fn(&A1, &mut A2, &mut A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &mut A2, &mut A3, A4)>
                    for value::HkFn<fn(&A1, &mut A2, &mut A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &mut A2, &mut A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &mut A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &mut A2, &mut A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, &mut A3, A4, A5)>
                        for value::HkFn<fn(&A1, &mut A2, &mut A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, &mut A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &mut A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, &mut A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, &mut A3, A4, &A5)>
                        for HkFn<fn(&A1, &mut A2, &mut A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, &mut A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &mut A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, &mut A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&A1, &mut A2, &mut A3, A4, &mut A5)>
                        for HkFn<fn(&A1, &mut A2, &mut A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, &mut A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &mut A2, &mut A3, &A4) -> Out,
                ) -> HkFn<fn(&A1, &mut A2, &mut A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &mut A2, &mut A3, &A4)>
                    for HkFn<fn(&A1, &mut A2, &mut A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &mut A2, &mut A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &mut A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &mut A2, &mut A3, &A4, A5) -> Out>
                    {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, &mut A3, &A4, A5)>
                        for value::HkFn<fn(&A1, &mut A2, &mut A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, &mut A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &mut A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, &mut A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&A1, &mut A2, &mut A3, &A4, &A5)>
                        for HkFn<fn(&A1, &mut A2, &mut A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, &mut A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &mut A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, &mut A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&A1, &mut A2, &mut A3, &A4, &mut A5)>
                        for HkFn<fn(&A1, &mut A2, &mut A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&A1, &mut A2, &mut A3, &A4, &mut A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&A1, &mut A2, &mut A3, &mut A4) -> Out,
                ) -> HkFn<fn(&A1, &mut A2, &mut A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&A1, &mut A2, &mut A3, &mut A4)>
                    for HkFn<fn(&A1, &mut A2, &mut A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&A1, &mut A2, &mut A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &mut A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&A1, &mut A2, &mut A3, &mut A4, A5) -> Out>
                    {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&A1, &mut A2, &mut A3, &mut A4, A5)>
                        for value::HkFn<fn(&A1, &mut A2, &mut A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&A1, &mut A2, &mut A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &mut A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, &mut A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&A1, &mut A2, &mut A3, &mut A4, &A5)>
                        for HkFn<fn(&A1, &mut A2, &mut A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&A1, &mut A2, &mut A3, &mut A4, &A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&A1, &mut A2, &mut A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, &mut A3, &mut A4, &mut A5) -> Out>
                    {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&A1, &mut A2, &mut A3, &mut A4, &mut A5)>
                        for HkFn<fn(&A1, &mut A2, &mut A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&A1, &mut A2, &mut A3, &mut A4, &mut A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
        }
    }
    pub fn r#mut<A1, Out>(f: fn(&mut A1) -> Out) -> HkFn<fn(&mut A1) -> Out> {
        HkFn(f)
    }
    impl<A1, Out> crate::cbk::Fn<(&mut A1,)> for HkFn<fn(&mut A1) -> Out> {
        type Output = Out;
        fn call(&self, args: (&mut A1,)) -> Self::Output {
            crate::cbk::Fn::<_>::call(&self.0, args)
        }
    }
    pub mod r#mut {
        use super::HkFn;
        pub fn value<A1, A2, Out>(
            f: fn(&mut A1, A2) -> Out,
        ) -> value::HkFn<fn(&mut A1, A2) -> Out> {
            value::HkFn(f)
        }
        impl<A1, A2, Out> crate::cbk::Fn<(&mut A1, A2)> for value::HkFn<fn(&mut A1, A2) -> Out> {
            type Output = Out;
            fn call(&self, args: (&mut A1, A2)) -> Self::Output {
                crate::cbk::Fn::<_>::call(&self.0, args)
            }
        }
        pub mod value {
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);

            pub fn value<A1, A2, A3, Out>(
                f: fn(&mut A1, A2, A3) -> Out,
            ) -> value::HkFn<fn(&mut A1, A2, A3) -> Out> {
                value::HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&mut A1, A2, A3)>
                for value::HkFn<fn(&mut A1, A2, A3) -> Out>
            {
                type Output = Out;
                fn call(&self, args: (&mut A1, A2, A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod value {
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);

                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, A2, A3, A4) -> Out,
                ) -> value::HkFn<fn(&mut A1, A2, A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, A2, A3, A4)>
                    for value::HkFn<fn(&mut A1, A2, A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, A2, A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, A2, A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, A3, A4, A5)>
                        for value::HkFn<fn(&mut A1, A2, A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, A3, A4, &A5)>
                        for HkFn<fn(&mut A1, A2, A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, A3, A4, &mut A5)>
                        for HkFn<fn(&mut A1, A2, A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, A2, A3, &A4) -> Out,
                ) -> HkFn<fn(&mut A1, A2, A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, A2, A3, &A4)>
                    for HkFn<fn(&mut A1, A2, A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, A2, A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, A2, A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, A3, &A4, A5)>
                        for value::HkFn<fn(&mut A1, A2, A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, A3, &A4, &A5)>
                        for HkFn<fn(&mut A1, A2, A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, A3, &A4, &mut A5)>
                        for HkFn<fn(&mut A1, A2, A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, A2, A3, &mut A4) -> Out,
                ) -> HkFn<fn(&mut A1, A2, A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, A2, A3, &mut A4)>
                    for HkFn<fn(&mut A1, A2, A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, A2, A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, A2, A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, A3, &mut A4, A5)>
                        for value::HkFn<fn(&mut A1, A2, A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, A3, &mut A4, &A5)>
                        for HkFn<fn(&mut A1, A2, A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, A2, A3, &mut A4, &mut A5)>
                        for HkFn<fn(&mut A1, A2, A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#ref<A1, A2, A3, Out>(
                f: fn(&mut A1, A2, &A3) -> Out,
            ) -> HkFn<fn(&mut A1, A2, &A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&mut A1, A2, &A3)> for HkFn<fn(&mut A1, A2, &A3) -> Out> {
                type Output = Out;
                fn call(&self, args: (&mut A1, A2, &A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#ref {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, A2, &A3, A4) -> Out,
                ) -> value::HkFn<fn(&mut A1, A2, &A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, A2, &A3, A4)>
                    for value::HkFn<fn(&mut A1, A2, &A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, A2, &A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, A2, &A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, &A3, A4, A5)>
                        for value::HkFn<fn(&mut A1, A2, &A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, &A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, &A3, A4, &A5)>
                        for HkFn<fn(&mut A1, A2, &A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, &A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, &A3, A4, &mut A5)>
                        for HkFn<fn(&mut A1, A2, &A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, A2, &A3, &A4) -> Out,
                ) -> HkFn<fn(&mut A1, A2, &A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, A2, &A3, &A4)>
                    for HkFn<fn(&mut A1, A2, &A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, A2, &A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, A2, &A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, &A3, &A4, A5)>
                        for value::HkFn<fn(&mut A1, A2, &A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, &A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, &A3, &A4, &A5)>
                        for HkFn<fn(&mut A1, A2, &A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, &A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, &A3, &A4, &mut A5)>
                        for HkFn<fn(&mut A1, A2, &A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, A2, &A3, &mut A4) -> Out,
                ) -> HkFn<fn(&mut A1, A2, &A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, A2, &A3, &mut A4)>
                    for HkFn<fn(&mut A1, A2, &A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, A2, &A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, A2, &A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, &A3, &mut A4, A5)>
                        for value::HkFn<fn(&mut A1, A2, &A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, &A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, &A3, &mut A4, &A5)>
                        for HkFn<fn(&mut A1, A2, &A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, &A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, A2, &A3, &mut A4, &mut A5)>
                        for HkFn<fn(&mut A1, A2, &A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#mut<A1, A2, A3, Out>(
                f: fn(&mut A1, A2, &mut A3) -> Out,
            ) -> HkFn<fn(&mut A1, A2, &mut A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&mut A1, A2, &mut A3)>
                for HkFn<fn(&mut A1, A2, &mut A3) -> Out>
            {
                type Output = Out;
                fn call(&self, args: (&mut A1, A2, &mut A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#mut {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, A2, &mut A3, A4) -> Out,
                ) -> value::HkFn<fn(&mut A1, A2, &mut A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, A2, &mut A3, A4)>
                    for value::HkFn<fn(&mut A1, A2, &mut A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, A2, &mut A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &mut A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, A2, &mut A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, &mut A3, A4, A5)>
                        for value::HkFn<fn(&mut A1, A2, &mut A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &mut A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &mut A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, &mut A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, &mut A3, A4, &A5)>
                        for HkFn<fn(&mut A1, A2, &mut A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &mut A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &mut A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, &mut A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, A2, &mut A3, A4, &mut A5)>
                        for HkFn<fn(&mut A1, A2, &mut A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &mut A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, A2, &mut A3, &A4) -> Out,
                ) -> HkFn<fn(&mut A1, A2, &mut A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, A2, &mut A3, &A4)>
                    for HkFn<fn(&mut A1, A2, &mut A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, A2, &mut A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &mut A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, A2, &mut A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, &mut A3, &A4, A5)>
                        for value::HkFn<fn(&mut A1, A2, &mut A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &mut A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &mut A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, &mut A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, A2, &mut A3, &A4, &A5)>
                        for HkFn<fn(&mut A1, A2, &mut A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &mut A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &mut A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, &mut A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, A2, &mut A3, &A4, &mut A5)>
                        for HkFn<fn(&mut A1, A2, &mut A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &mut A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, A2, &mut A3, &mut A4) -> Out,
                ) -> HkFn<fn(&mut A1, A2, &mut A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, A2, &mut A3, &mut A4)>
                    for HkFn<fn(&mut A1, A2, &mut A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, A2, &mut A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &mut A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, A2, &mut A3, &mut A4, A5) -> Out>
                    {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, A2, &mut A3, &mut A4, A5)>
                        for value::HkFn<fn(&mut A1, A2, &mut A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &mut A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &mut A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, &mut A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, A2, &mut A3, &mut A4, &A5)>
                        for HkFn<fn(&mut A1, A2, &mut A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, A2, &mut A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, A2, &mut A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, &mut A3, &mut A4, &mut A5) -> Out>
                    {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, A2, &mut A3, &mut A4, &mut A5)>
                        for HkFn<fn(&mut A1, A2, &mut A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&mut A1, A2, &mut A3, &mut A4, &mut A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
        }
        pub fn r#ref<A1, A2, Out>(f: fn(&mut A1, &A2) -> Out) -> HkFn<fn(&mut A1, &A2) -> Out> {
            HkFn(f)
        }
        impl<A1, A2, Out> crate::cbk::Fn<(&mut A1, &A2)> for HkFn<fn(&mut A1, &A2) -> Out> {
            type Output = Out;
            fn call(&self, args: (&mut A1, &A2)) -> Self::Output {
                crate::cbk::Fn::<_>::call(&self.0, args)
            }
        }
        pub mod r#ref {
            use super::HkFn;
            pub fn value<A1, A2, A3, Out>(
                f: fn(&mut A1, &A2, A3) -> Out,
            ) -> value::HkFn<fn(&mut A1, &A2, A3) -> Out> {
                value::HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&mut A1, &A2, A3)>
                for value::HkFn<fn(&mut A1, &A2, A3) -> Out>
            {
                type Output = Out;
                fn call(&self, args: (&mut A1, &A2, A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod value {
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);

                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &A2, A3, A4) -> Out,
                ) -> value::HkFn<fn(&mut A1, &A2, A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &A2, A3, A4)>
                    for value::HkFn<fn(&mut A1, &A2, A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &A2, A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &A2, A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, A3, A4, A5)>
                        for value::HkFn<fn(&mut A1, &A2, A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, A3, A4, &A5)>
                        for HkFn<fn(&mut A1, &A2, A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, A3, A4, &mut A5)>
                        for HkFn<fn(&mut A1, &A2, A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &A2, A3, &A4) -> Out,
                ) -> HkFn<fn(&mut A1, &A2, A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &A2, A3, &A4)>
                    for HkFn<fn(&mut A1, &A2, A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &A2, A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &A2, A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, A3, &A4, A5)>
                        for value::HkFn<fn(&mut A1, &A2, A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, A3, &A4, &A5)>
                        for HkFn<fn(&mut A1, &A2, A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, A3, &A4, &mut A5)>
                        for HkFn<fn(&mut A1, &A2, A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &A2, A3, &mut A4) -> Out,
                ) -> HkFn<fn(&mut A1, &A2, A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &A2, A3, &mut A4)>
                    for HkFn<fn(&mut A1, &A2, A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &A2, A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &A2, A3, &mut A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, A3, &mut A4, A5)>
                        for value::HkFn<fn(&mut A1, &A2, A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, A3, &mut A4, &A5)>
                        for HkFn<fn(&mut A1, &A2, A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &A2, A3, &mut A4, &mut A5)>
                        for HkFn<fn(&mut A1, &A2, A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, A3, &mut A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#ref<A1, A2, A3, Out>(
                f: fn(&mut A1, &A2, &A3) -> Out,
            ) -> HkFn<fn(&mut A1, &A2, &A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&mut A1, &A2, &A3)> for HkFn<fn(&mut A1, &A2, &A3) -> Out> {
                type Output = Out;
                fn call(&self, args: (&mut A1, &A2, &A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#ref {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &A2, &A3, A4) -> Out,
                ) -> value::HkFn<fn(&mut A1, &A2, &A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &A2, &A3, A4)>
                    for value::HkFn<fn(&mut A1, &A2, &A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &A2, &A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &A2, &A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, &A3, A4, A5)>
                        for value::HkFn<fn(&mut A1, &A2, &A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, &A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, &A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, &A3, A4, &A5)>
                        for HkFn<fn(&mut A1, &A2, &A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, &A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, &A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, &A3, A4, &mut A5)>
                        for HkFn<fn(&mut A1, &A2, &A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, &A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &A2, &A3, &A4) -> Out,
                ) -> HkFn<fn(&mut A1, &A2, &A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &A2, &A3, &A4)>
                    for HkFn<fn(&mut A1, &A2, &A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &A2, &A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &A2, &A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, &A3, &A4, A5)>
                        for value::HkFn<fn(&mut A1, &A2, &A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, &A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, &A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, &A3, &A4, &A5)>
                        for HkFn<fn(&mut A1, &A2, &A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, &A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, &A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, &A3, &A4, &mut A5)>
                        for HkFn<fn(&mut A1, &A2, &A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, &A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &A2, &A3, &mut A4) -> Out,
                ) -> HkFn<fn(&mut A1, &A2, &A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &A2, &A3, &mut A4)>
                    for HkFn<fn(&mut A1, &A2, &A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &A2, &A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &A2, &A3, &mut A4, A5) -> Out>
                    {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, &A3, &mut A4, A5)>
                        for value::HkFn<fn(&mut A1, &A2, &A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, &A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, &A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, &A3, &mut A4, &A5)>
                        for HkFn<fn(&mut A1, &A2, &A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, &A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, &A3, &mut A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &A2, &A3, &mut A4, &mut A5)>
                        for HkFn<fn(&mut A1, &A2, &A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&mut A1, &A2, &A3, &mut A4, &mut A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#mut<A1, A2, A3, Out>(
                f: fn(&mut A1, &A2, &mut A3) -> Out,
            ) -> HkFn<fn(&mut A1, &A2, &mut A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&mut A1, &A2, &mut A3)>
                for HkFn<fn(&mut A1, &A2, &mut A3) -> Out>
            {
                type Output = Out;
                fn call(&self, args: (&mut A1, &A2, &mut A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#mut {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &A2, &mut A3, A4) -> Out,
                ) -> value::HkFn<fn(&mut A1, &A2, &mut A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &A2, &mut A3, A4)>
                    for value::HkFn<fn(&mut A1, &A2, &mut A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &A2, &mut A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &mut A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &A2, &mut A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, &mut A3, A4, A5)>
                        for value::HkFn<fn(&mut A1, &A2, &mut A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, &mut A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &mut A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, &mut A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, &mut A3, A4, &A5)>
                        for HkFn<fn(&mut A1, &A2, &mut A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, &mut A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &mut A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, &mut A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &A2, &mut A3, A4, &mut A5)>
                        for HkFn<fn(&mut A1, &A2, &mut A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, &mut A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &A2, &mut A3, &A4) -> Out,
                ) -> HkFn<fn(&mut A1, &A2, &mut A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &A2, &mut A3, &A4)>
                    for HkFn<fn(&mut A1, &A2, &mut A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &A2, &mut A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &mut A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &A2, &mut A3, &A4, A5) -> Out>
                    {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, &mut A3, &A4, A5)>
                        for value::HkFn<fn(&mut A1, &A2, &mut A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, &mut A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &mut A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, &mut A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &A2, &mut A3, &A4, &A5)>
                        for HkFn<fn(&mut A1, &A2, &mut A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, &mut A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &mut A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, &mut A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &A2, &mut A3, &A4, &mut A5)>
                        for HkFn<fn(&mut A1, &A2, &mut A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&mut A1, &A2, &mut A3, &A4, &mut A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &A2, &mut A3, &mut A4) -> Out,
                ) -> HkFn<fn(&mut A1, &A2, &mut A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &A2, &mut A3, &mut A4)>
                    for HkFn<fn(&mut A1, &A2, &mut A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &A2, &mut A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &mut A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &A2, &mut A3, &mut A4, A5) -> Out>
                    {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &A2, &mut A3, &mut A4, A5)>
                        for value::HkFn<fn(&mut A1, &A2, &mut A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &A2, &mut A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &mut A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, &mut A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &A2, &mut A3, &mut A4, &A5)>
                        for HkFn<fn(&mut A1, &A2, &mut A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&mut A1, &A2, &mut A3, &mut A4, &A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &A2, &mut A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, &mut A3, &mut A4, &mut A5) -> Out>
                    {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &A2, &mut A3, &mut A4, &mut A5)>
                        for HkFn<fn(&mut A1, &A2, &mut A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&mut A1, &A2, &mut A3, &mut A4, &mut A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
        }
        pub fn r#mut<A1, A2, Out>(
            f: fn(&mut A1, &mut A2) -> Out,
        ) -> HkFn<fn(&mut A1, &mut A2) -> Out> {
            HkFn(f)
        }
        impl<A1, A2, Out> crate::cbk::Fn<(&mut A1, &mut A2)> for HkFn<fn(&mut A1, &mut A2) -> Out> {
            type Output = Out;
            fn call(&self, args: (&mut A1, &mut A2)) -> Self::Output {
                crate::cbk::Fn::<_>::call(&self.0, args)
            }
        }
        pub mod r#mut {
            use super::HkFn;
            pub fn value<A1, A2, A3, Out>(
                f: fn(&mut A1, &mut A2, A3) -> Out,
            ) -> value::HkFn<fn(&mut A1, &mut A2, A3) -> Out> {
                value::HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&mut A1, &mut A2, A3)>
                for value::HkFn<fn(&mut A1, &mut A2, A3) -> Out>
            {
                type Output = Out;
                fn call(&self, args: (&mut A1, &mut A2, A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod value {
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);

                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &mut A2, A3, A4) -> Out,
                ) -> value::HkFn<fn(&mut A1, &mut A2, A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &mut A2, A3, A4)>
                    for value::HkFn<fn(&mut A1, &mut A2, A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &mut A2, A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &mut A2, A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &mut A2, A3, A4, A5)>
                        for value::HkFn<fn(&mut A1, &mut A2, A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &mut A2, A3, A4, &A5)>
                        for HkFn<fn(&mut A1, &mut A2, A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, A3, A4, &mut A5)>
                        for HkFn<fn(&mut A1, &mut A2, A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &mut A2, A3, &A4) -> Out,
                ) -> HkFn<fn(&mut A1, &mut A2, A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &mut A2, A3, &A4)>
                    for HkFn<fn(&mut A1, &mut A2, A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &mut A2, A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &mut A2, A3, &A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &mut A2, A3, &A4, A5)>
                        for value::HkFn<fn(&mut A1, &mut A2, A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &mut A2, A3, &A4, &A5)>
                        for HkFn<fn(&mut A1, &mut A2, A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, A3, &A4, &mut A5)>
                        for HkFn<fn(&mut A1, &mut A2, A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, A3, &A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &mut A2, A3, &mut A4) -> Out,
                ) -> HkFn<fn(&mut A1, &mut A2, A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &mut A2, A3, &mut A4)>
                    for HkFn<fn(&mut A1, &mut A2, A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &mut A2, A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &mut A2, A3, &mut A4, A5) -> Out>
                    {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, A3, &mut A4, A5)>
                        for value::HkFn<fn(&mut A1, &mut A2, A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, A3, &mut A4, &A5)>
                        for HkFn<fn(&mut A1, &mut A2, A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, A3, &mut A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, A3, &mut A4, &mut A5) -> Out>
                    {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, A3, &mut A4, &mut A5)>
                        for HkFn<fn(&mut A1, &mut A2, A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&mut A1, &mut A2, A3, &mut A4, &mut A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#ref<A1, A2, A3, Out>(
                f: fn(&mut A1, &mut A2, &A3) -> Out,
            ) -> HkFn<fn(&mut A1, &mut A2, &A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&mut A1, &mut A2, &A3)>
                for HkFn<fn(&mut A1, &mut A2, &A3) -> Out>
            {
                type Output = Out;
                fn call(&self, args: (&mut A1, &mut A2, &A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#ref {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &mut A2, &A3, A4) -> Out,
                ) -> value::HkFn<fn(&mut A1, &mut A2, &A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &mut A2, &A3, A4)>
                    for value::HkFn<fn(&mut A1, &mut A2, &A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &mut A2, &A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &mut A2, &A3, A4, A5) -> Out> {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &mut A2, &A3, A4, A5)>
                        for value::HkFn<fn(&mut A1, &mut A2, &A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, &A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, &A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &mut A2, &A3, A4, &A5)>
                        for HkFn<fn(&mut A1, &mut A2, &A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, &A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, &A3, A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, &A3, A4, &mut A5)>
                        for HkFn<fn(&mut A1, &mut A2, &A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, &A3, A4, &mut A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &mut A2, &A3, &A4) -> Out,
                ) -> HkFn<fn(&mut A1, &mut A2, &A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &mut A2, &A3, &A4)>
                    for HkFn<fn(&mut A1, &mut A2, &A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &mut A2, &A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &mut A2, &A3, &A4, A5) -> Out>
                    {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &mut A2, &A3, &A4, A5)>
                        for value::HkFn<fn(&mut A1, &mut A2, &A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, &A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, &A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out> crate::cbk::Fn<(&mut A1, &mut A2, &A3, &A4, &A5)>
                        for HkFn<fn(&mut A1, &mut A2, &A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, &A3, &A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, &A3, &A4, &mut A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, &A3, &A4, &mut A5)>
                        for HkFn<fn(&mut A1, &mut A2, &A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&mut A1, &mut A2, &A3, &A4, &mut A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &mut A2, &A3, &mut A4) -> Out,
                ) -> HkFn<fn(&mut A1, &mut A2, &A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &mut A2, &A3, &mut A4)>
                    for HkFn<fn(&mut A1, &mut A2, &A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &mut A2, &A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &mut A2, &A3, &mut A4, A5) -> Out>
                    {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, &A3, &mut A4, A5)>
                        for value::HkFn<fn(&mut A1, &mut A2, &A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, &A3, &mut A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, &A3, &mut A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, &A3, &mut A4, &A5)>
                        for HkFn<fn(&mut A1, &mut A2, &A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&mut A1, &mut A2, &A3, &mut A4, &A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, &A3, &mut A4, &mut A5) -> Out>
                    {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, &A3, &mut A4, &mut A5)>
                        for HkFn<fn(&mut A1, &mut A2, &A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&mut A1, &mut A2, &A3, &mut A4, &mut A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
            pub fn r#mut<A1, A2, A3, Out>(
                f: fn(&mut A1, &mut A2, &mut A3) -> Out,
            ) -> HkFn<fn(&mut A1, &mut A2, &mut A3) -> Out> {
                HkFn(f)
            }
            impl<A1, A2, A3, Out> crate::cbk::Fn<(&mut A1, &mut A2, &mut A3)>
                for HkFn<fn(&mut A1, &mut A2, &mut A3) -> Out>
            {
                type Output = Out;
                fn call(&self, args: (&mut A1, &mut A2, &mut A3)) -> Self::Output {
                    crate::cbk::Fn::<_>::call(&self.0, args)
                }
            }
            pub mod r#mut {
                use super::HkFn;
                pub fn value<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &mut A2, &mut A3, A4) -> Out,
                ) -> value::HkFn<fn(&mut A1, &mut A2, &mut A3, A4) -> Out> {
                    value::HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &mut A2, &mut A3, A4)>
                    for value::HkFn<fn(&mut A1, &mut A2, &mut A3, A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &mut A2, &mut A3, A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod value {
                    #[derive(Debug, Clone, Copy)]
                    pub struct HkFn<F>(pub(super) F);

                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &mut A3, A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &mut A2, &mut A3, A4, A5) -> Out>
                    {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, &mut A3, A4, A5)>
                        for value::HkFn<fn(&mut A1, &mut A2, &mut A3, A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, &mut A3, A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &mut A3, A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, &mut A3, A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, &mut A3, A4, &A5)>
                        for HkFn<fn(&mut A1, &mut A2, &mut A3, A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, &mut A3, A4, &A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &mut A3, A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, &mut A3, A4, &mut A5) -> Out>
                    {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, &mut A3, A4, &mut A5)>
                        for HkFn<fn(&mut A1, &mut A2, &mut A3, A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&mut A1, &mut A2, &mut A3, A4, &mut A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#ref<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &mut A2, &mut A3, &A4) -> Out,
                ) -> HkFn<fn(&mut A1, &mut A2, &mut A3, &A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &mut A2, &mut A3, &A4)>
                    for HkFn<fn(&mut A1, &mut A2, &mut A3, &A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &mut A2, &mut A3, &A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#ref {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &mut A3, &A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &mut A2, &mut A3, &A4, A5) -> Out>
                    {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, &mut A3, &A4, A5)>
                        for value::HkFn<fn(&mut A1, &mut A2, &mut A3, &A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(&self, args: (&mut A1, &mut A2, &mut A3, &A4, A5)) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &mut A3, &A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, &mut A3, &A4, &A5) -> Out> {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, &mut A3, &A4, &A5)>
                        for HkFn<fn(&mut A1, &mut A2, &mut A3, &A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&mut A1, &mut A2, &mut A3, &A4, &A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &mut A3, &A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, &mut A3, &A4, &mut A5) -> Out>
                    {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, &mut A3, &A4, &mut A5)>
                        for HkFn<fn(&mut A1, &mut A2, &mut A3, &A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&mut A1, &mut A2, &mut A3, &A4, &mut A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
                pub fn r#mut<A1, A2, A3, A4, Out>(
                    f: fn(&mut A1, &mut A2, &mut A3, &mut A4) -> Out,
                ) -> HkFn<fn(&mut A1, &mut A2, &mut A3, &mut A4) -> Out> {
                    HkFn(f)
                }
                impl<A1, A2, A3, A4, Out> crate::cbk::Fn<(&mut A1, &mut A2, &mut A3, &mut A4)>
                    for HkFn<fn(&mut A1, &mut A2, &mut A3, &mut A4) -> Out>
                {
                    type Output = Out;
                    fn call(&self, args: (&mut A1, &mut A2, &mut A3, &mut A4)) -> Self::Output {
                        crate::cbk::Fn::<_>::call(&self.0, args)
                    }
                }
                pub mod r#mut {
                    use super::HkFn;
                    pub fn value<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &mut A3, &mut A4, A5) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &mut A2, &mut A3, &mut A4, A5) -> Out>
                    {
                        value::HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, &mut A3, &mut A4, A5)>
                        for value::HkFn<fn(&mut A1, &mut A2, &mut A3, &mut A4, A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&mut A1, &mut A2, &mut A3, &mut A4, A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod value {
                        #[derive(Debug, Clone, Copy)]
                        pub struct HkFn<F>(pub(super) F);
                    }
                    pub fn r#ref<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &mut A3, &mut A4, &A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, &mut A3, &mut A4, &A5) -> Out>
                    {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, &mut A3, &mut A4, &A5)>
                        for HkFn<fn(&mut A1, &mut A2, &mut A3, &mut A4, &A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&mut A1, &mut A2, &mut A3, &mut A4, &A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#ref {}
                    pub fn r#mut<A1, A2, A3, A4, A5, Out>(
                        f: fn(&mut A1, &mut A2, &mut A3, &mut A4, &mut A5) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, &mut A3, &mut A4, &mut A5) -> Out>
                    {
                        HkFn(f)
                    }
                    impl<A1, A2, A3, A4, A5, Out>
                        crate::cbk::Fn<(&mut A1, &mut A2, &mut A3, &mut A4, &mut A5)>
                        for HkFn<fn(&mut A1, &mut A2, &mut A3, &mut A4, &mut A5) -> Out>
                    {
                        type Output = Out;
                        fn call(
                            &self,
                            args: (&mut A1, &mut A2, &mut A3, &mut A4, &mut A5),
                        ) -> Self::Output {
                            crate::cbk::Fn::<_>::call(&self.0, args)
                        }
                    }
                    pub mod r#mut {}
                }
            }
        }
    }
}

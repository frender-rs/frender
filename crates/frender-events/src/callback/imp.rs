// Recursive expansion of impl_with_macro_rules! macro
// ====================================================

use super::HkFn;
impl crate::callback::sealed::Tuple for () {}

impl<'arg> crate::callback::argument::Arguments<'arg> for () {
    type Arguments = ();
}
impl crate::callback::argument::ArgumentTypes for () {
    type First = crate::callback::argument::Invalid;
    type FirstTrimmed = ();
    type Last = crate::callback::argument::Invalid;
    type LastTrimmed = crate::callback::argument::InvalidTuple;
}
impl<ArgType: for<'a> crate::callback::argument::ArgumentType<'a>>
    crate::callback::argument::ArgumentTypesAppend<ArgType> for ()
{
    type Appended = (ArgType,);
    fn append<'a>(
        (): crate::callback::argument::ArgumentsOfTypes<'a, Self>,
        last: <ArgType as crate::callback::argument::ArgumentType<'a>>::Argument,
    ) -> crate::callback::argument::ArgumentsOfTypes<'a, Self::Appended> {
        (last,)
    }
}
impl<Out> crate::callback::Callable<()> for fn() -> Out {
    type Output = Out;
    fn call_fn(&self, (): ()) -> Self::Output {
        self()
    }
}
impl<A1> crate::callback::sealed::Tuple for (A1,) {}

impl<'arg, A1: for<'a> crate::callback::argument::ArgumentType<'a>>
    crate::callback::argument::Arguments<'arg> for (A1,)
{
    type Arguments = (<A1 as crate::callback::argument::ArgumentType<'arg>>::Argument,);
}
impl<A1: for<'a> crate::callback::argument::ArgumentType<'a>>
    crate::callback::argument::ArgumentTypes for (A1,)
{
    type First = A1;
    type FirstTrimmed = ();
    type Last = A1;
    type LastTrimmed = ();
}
impl<
        ArgType: for<'a> crate::callback::argument::ArgumentType<'a>,
        A1: for<'a> crate::callback::argument::ArgumentType<'a>,
    > crate::callback::argument::ArgumentTypesAppend<ArgType> for (A1,)
{
    type Appended = (A1, ArgType);
    fn append<'a>(
        (a1,): crate::callback::argument::ArgumentsOfTypes<'a, Self>,
        last: <ArgType as crate::callback::argument::ArgumentType<'a>>::Argument,
    ) -> crate::callback::argument::ArgumentsOfTypes<'a, Self::Appended> {
        (a1, last)
    }
}
impl<A1, Out> crate::callback::Callable<(A1,)> for fn(A1) -> Out {
    type Output = Out;
    fn call_fn(&self, (a1,): (A1,)) -> Self::Output {
        self(a1)
    }
}
impl<A1, A2> crate::callback::sealed::Tuple for (A1, A2) {}

impl<
        'arg,
        A1: for<'a> crate::callback::argument::ArgumentType<'a>,
        A2: for<'a> crate::callback::argument::ArgumentType<'a>,
    > crate::callback::argument::Arguments<'arg> for (A1, A2)
{
    type Arguments = (
        <A1 as crate::callback::argument::ArgumentType<'arg>>::Argument,
        <A2 as crate::callback::argument::ArgumentType<'arg>>::Argument,
    );
}
impl<
        A1: for<'a> crate::callback::argument::ArgumentType<'a>,
        A2: for<'a> crate::callback::argument::ArgumentType<'a>,
    > crate::callback::argument::ArgumentTypes for (A1, A2)
{
    type First = A1;
    type FirstTrimmed = (A2,);
    type Last = A2;
    type LastTrimmed = ();
}
impl<
        ArgType: for<'a> crate::callback::argument::ArgumentType<'a>,
        A1: for<'a> crate::callback::argument::ArgumentType<'a>,
        A2: for<'a> crate::callback::argument::ArgumentType<'a>,
    > crate::callback::argument::ArgumentTypesAppend<ArgType> for (A1, A2)
{
    type Appended = (A1, A2, ArgType);
    fn append<'a>(
        (a1, a2): crate::callback::argument::ArgumentsOfTypes<'a, Self>,
        last: <ArgType as crate::callback::argument::ArgumentType<'a>>::Argument,
    ) -> crate::callback::argument::ArgumentsOfTypes<'a, Self::Appended> {
        (a1, a2, last)
    }
}
impl<A1, A2, Out> crate::callback::Callable<(A1, A2)> for fn(A1, A2) -> Out {
    type Output = Out;
    fn call_fn(&self, (a1, a2): (A1, A2)) -> Self::Output {
        self(a1, a2)
    }
}
impl<A1, A2, A3> crate::callback::sealed::Tuple for (A1, A2, A3) {}

impl<
        'arg,
        A1: for<'a> crate::callback::argument::ArgumentType<'a>,
        A2: for<'a> crate::callback::argument::ArgumentType<'a>,
        A3: for<'a> crate::callback::argument::ArgumentType<'a>,
    > crate::callback::argument::Arguments<'arg> for (A1, A2, A3)
{
    type Arguments = (
        <A1 as crate::callback::argument::ArgumentType<'arg>>::Argument,
        <A2 as crate::callback::argument::ArgumentType<'arg>>::Argument,
        <A3 as crate::callback::argument::ArgumentType<'arg>>::Argument,
    );
}
impl<
        A1: for<'a> crate::callback::argument::ArgumentType<'a>,
        A2: for<'a> crate::callback::argument::ArgumentType<'a>,
        A3: for<'a> crate::callback::argument::ArgumentType<'a>,
    > crate::callback::argument::ArgumentTypes for (A1, A2, A3)
{
    type First = A1;
    type FirstTrimmed = (A2, A3);
    type Last = A3;
    type LastTrimmed = ();
}
impl<A1, A2, A3, Out> crate::callback::Callable<(A1, A2, A3)> for fn(A1, A2, A3) -> Out {
    type Output = Out;
    fn call_fn(&self, (a1, a2, a3): (A1, A2, A3)) -> Self::Output {
        self(a1, a2, a3)
    }
}
pub fn value<A1, Out>(f: fn(A1) -> Out) -> fn(A1) -> Out {
    f
}
impl<A1, Out> crate::callback::IsCallable for fn(A1) -> Out {}

pub mod value {
    #[derive(Debug, Clone, Copy)]
    pub struct HkFn<F>(pub(super) F);

    pub fn value<A1, A2, Out>(f: fn(A1, A2) -> Out) -> fn(A1, A2) -> Out {
        f
    }
    impl<A1, A2, Out> crate::callback::IsCallable for fn(A1, A2) -> Out {}

    pub mod value {
        #[derive(Debug, Clone, Copy)]
        pub struct HkFn<F>(pub(super) F);

        pub fn value<A1, A2, A3, Out>(f: fn(A1, A2, A3) -> Out) -> fn(A1, A2, A3) -> Out {
            f
        }
        impl<A1, A2, A3, Out> crate::callback::IsCallable for fn(A1, A2, A3) -> Out {}

        pub mod value {
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);
        }
        impl<A1, A2, A3, Out> crate::callback::CallableWithFixedArguments for fn(A1, A2, A3) -> Out {
            type FixedArgumentTypes = (
                crate::callback::argument::Value<A1>,
                crate::callback::argument::Value<A2>,
                crate::callback::argument::Value<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1, A2, A3, Out, Arg> crate::callback::Callable<(A1, A2)>
            for crate::callback::argument::LastArgumentProvided<fn(A1, A2, A3) -> Out, Arg>
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::value<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (A1, A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#ref<A1, A2, A3: ?Sized, Out>(
            f: fn(A1, A2, &A3) -> Out,
        ) -> HkFn<fn(A1, A2, &A3) -> Out> {
            HkFn(f)
        }
        impl<A1, A2, A3: ?Sized, Out> crate::callback::Callable<(A1, A2, &A3)>
            for HkFn<fn(A1, A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (A1, A2, &A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1, A2, A3: ?Sized, Out> PartialEq for HkFn<fn(A1, A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1, A2, A3: ?Sized, Out> crate::callback::IsCallable for HkFn<fn(A1, A2, &A3) -> Out> {}

        pub mod r#ref {
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1, A2, A3, Out>(
                    f: fn(A1, A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::callback::argument::LastArgumentProvided<
                    HkFn<fn(A1, A2, &A3) -> Out>,
                    crate::callback::argument::Refed<A3>,
                > {
                    use crate::callback::CallableWithFixedArguments;
                    super::super::r#ref(f)
                        .provide_last_argument(crate::callback::argument::Refed(arg))
                }
            }
            pub use provide_last_argument::provide_last_argument;
        }
        impl<A1, A2, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(A1, A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::Value<A1>,
                crate::callback::argument::Value<A2>,
                crate::callback::argument::ByRef<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1, A2, A3: ?Sized, Out, Arg> crate::callback::Callable<(A1, A2)>
            for crate::callback::argument::LastArgumentProvided<HkFn<fn(A1, A2, &A3) -> Out>, Arg>
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#ref<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (A1, A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#mut<A1, A2, A3: ?Sized, Out>(
            f: fn(A1, A2, &mut A3) -> Out,
        ) -> HkFn<fn(A1, A2, &mut A3) -> Out> {
            HkFn(f)
        }
        impl<A1, A2, A3: ?Sized, Out> crate::callback::Callable<(A1, A2, &mut A3)>
            for HkFn<fn(A1, A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (A1, A2, &mut A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1, A2, A3: ?Sized, Out> PartialEq for HkFn<fn(A1, A2, &mut A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1, A2, A3: ?Sized, Out> crate::callback::IsCallable for HkFn<fn(A1, A2, &mut A3) -> Out> {}

        pub mod r#mut {}
        impl<A1, A2, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(A1, A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::Value<A1>,
                crate::callback::argument::Value<A2>,
                crate::callback::argument::ByMut<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1, A2, A3: ?Sized, Out, Arg> crate::callback::Callable<(A1, A2)>
            for crate::callback::argument::LastArgumentProvided<
                HkFn<fn(A1, A2, &mut A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#mut<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (A1, A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
    }
    impl<A1, A2, Out> crate::callback::CallableWithFixedArguments for fn(A1, A2) -> Out {
        type FixedArgumentTypes = (
            crate::callback::argument::Value<A1>,
            crate::callback::argument::Value<A2>,
        );
    }
    #[cfg(aa)]
    impl<A1, A2, Out, Arg> crate::callback::Callable<(A1,)>
        for crate::callback::argument::LastArgumentProvided<fn(A1, A2) -> Out, Arg>
    where
        Arg: crate::callback::argument::ProvideArgument<
            ProvideArgumentType = crate::callback::argument::value<A2>,
        >,
    {
        type Output = Out;
        fn call_fn(&self, (A1,): (A1,)) -> Self::Output {
            self.last_argument.provide_argument_to(|last_argument| {
                crate::callback::Callable::<_>::call_fn(&self.f, (A1, last_argument))
            })
        }
    }
    pub fn r#ref<A1, A2: ?Sized, Out>(f: fn(A1, &A2) -> Out) -> HkFn<fn(A1, &A2) -> Out> {
        HkFn(f)
    }
    impl<A1, A2: ?Sized, Out> crate::callback::Callable<(A1, &A2)> for HkFn<fn(A1, &A2) -> Out> {
        type Output = Out;
        fn call_fn(&self, (A1, A2): (A1, &A2)) -> Self::Output {
            self.0(A1, A2)
        }
    }
    impl<A1, A2: ?Sized, Out> PartialEq for HkFn<fn(A1, &A2) -> Out> {
        fn eq(&self, other: &Self) -> bool {
            self.0 as usize == other.0 as usize
        }
    }
    impl<A1, A2: ?Sized, Out> crate::callback::IsCallable for HkFn<fn(A1, &A2) -> Out> {}

    pub mod r#ref {
        mod provide_last_argument {
            use super::super::HkFn;
            pub fn provide_last_argument<A1, A2, Out>(
                f: fn(A1, &A2) -> Out,
                arg: A2,
            ) -> crate::callback::argument::LastArgumentProvided<
                HkFn<fn(A1, &A2) -> Out>,
                crate::callback::argument::Refed<A2>,
            > {
                use crate::callback::CallableWithFixedArguments;
                super::super::r#ref(f).provide_last_argument(crate::callback::argument::Refed(arg))
            }
        }
        use super::HkFn;
        pub use provide_last_argument::provide_last_argument;
        pub fn value<A1, A2: ?Sized, A3, Out>(
            f: fn(A1, &A2, A3) -> Out,
        ) -> value::HkFn<fn(A1, &A2, A3) -> Out> {
            value::HkFn(f)
        }
        impl<A1, A2: ?Sized, A3, Out> crate::callback::Callable<(A1, &A2, A3)>
            for value::HkFn<fn(A1, &A2, A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (A1, &A2, A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1, A2: ?Sized, A3, Out> PartialEq for value::HkFn<fn(A1, &A2, A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1, A2: ?Sized, A3, Out> crate::callback::IsCallable for value::HkFn<fn(A1, &A2, A3) -> Out> {}

        pub mod value {
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);
        }
        impl<A1, A2: ?Sized, A3, Out> crate::callback::CallableWithFixedArguments
            for value::HkFn<fn(A1, &A2, A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::Value<A1>,
                crate::callback::argument::ByRef<A2>,
                crate::callback::argument::Value<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1, A2: ?Sized, A3, Out, Arg> crate::callback::Callable<(A1, &A2)>
            for crate::callback::argument::LastArgumentProvided<
                value::HkFn<fn(A1, &A2, A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::value<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (A1, &A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#ref<A1, A2: ?Sized, A3: ?Sized, Out>(
            f: fn(A1, &A2, &A3) -> Out,
        ) -> HkFn<fn(A1, &A2, &A3) -> Out> {
            HkFn(f)
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::callback::Callable<(A1, &A2, &A3)>
            for HkFn<fn(A1, &A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (A1, &A2, &A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(A1, &A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::callback::IsCallable
            for HkFn<fn(A1, &A2, &A3) -> Out>
        {
        }

        pub mod r#ref {
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1, A2: ?Sized, A3, Out>(
                    f: fn(A1, &A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::callback::argument::LastArgumentProvided<
                    HkFn<fn(A1, &A2, &A3) -> Out>,
                    crate::callback::argument::Refed<A3>,
                > {
                    use crate::callback::CallableWithFixedArguments;
                    super::super::r#ref(f)
                        .provide_last_argument(crate::callback::argument::Refed(arg))
                }
            }
            pub use provide_last_argument::provide_last_argument;
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(A1, &A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::Value<A1>,
                crate::callback::argument::ByRef<A2>,
                crate::callback::argument::ByRef<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1, A2: ?Sized, A3: ?Sized, Out, Arg> crate::callback::Callable<(A1, &A2)>
            for crate::callback::argument::LastArgumentProvided<HkFn<fn(A1, &A2, &A3) -> Out>, Arg>
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#ref<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (A1, &A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#mut<A1, A2: ?Sized, A3: ?Sized, Out>(
            f: fn(A1, &A2, &mut A3) -> Out,
        ) -> HkFn<fn(A1, &A2, &mut A3) -> Out> {
            HkFn(f)
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::callback::Callable<(A1, &A2, &mut A3)>
            for HkFn<fn(A1, &A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (A1, &A2, &mut A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(A1, &A2, &mut A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::callback::IsCallable
            for HkFn<fn(A1, &A2, &mut A3) -> Out>
        {
        }

        pub mod r#mut {}
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(A1, &A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::Value<A1>,
                crate::callback::argument::ByRef<A2>,
                crate::callback::argument::ByMut<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1, A2: ?Sized, A3: ?Sized, Out, Arg> crate::callback::Callable<(A1, &A2)>
            for crate::callback::argument::LastArgumentProvided<
                HkFn<fn(A1, &A2, &mut A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#mut<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (A1, &A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
    }
    impl<A1, A2: ?Sized, Out> crate::callback::CallableWithFixedArguments for HkFn<fn(A1, &A2) -> Out> {
        type FixedArgumentTypes = (
            crate::callback::argument::Value<A1>,
            crate::callback::argument::ByRef<A2>,
        );
    }
    #[cfg(aa)]
    impl<A1, A2: ?Sized, Out, Arg> crate::callback::Callable<(A1,)>
        for crate::callback::argument::LastArgumentProvided<HkFn<fn(A1, &A2) -> Out>, Arg>
    where
        Arg: crate::callback::argument::ProvideArgument<
            ProvideArgumentType = crate::callback::argument::r#ref<A2>,
        >,
    {
        type Output = Out;
        fn call_fn(&self, (A1,): (A1,)) -> Self::Output {
            self.last_argument.provide_argument_to(|last_argument| {
                crate::callback::Callable::<_>::call_fn(&self.f, (A1, last_argument))
            })
        }
    }
    pub fn r#mut<A1, A2: ?Sized, Out>(f: fn(A1, &mut A2) -> Out) -> HkFn<fn(A1, &mut A2) -> Out> {
        HkFn(f)
    }
    impl<A1, A2: ?Sized, Out> crate::callback::Callable<(A1, &mut A2)>
        for HkFn<fn(A1, &mut A2) -> Out>
    {
        type Output = Out;
        fn call_fn(&self, (A1, A2): (A1, &mut A2)) -> Self::Output {
            self.0(A1, A2)
        }
    }
    impl<A1, A2: ?Sized, Out> PartialEq for HkFn<fn(A1, &mut A2) -> Out> {
        fn eq(&self, other: &Self) -> bool {
            self.0 as usize == other.0 as usize
        }
    }
    impl<A1, A2: ?Sized, Out> crate::callback::IsCallable for HkFn<fn(A1, &mut A2) -> Out> {}

    pub mod r#mut {
        use super::HkFn;
        pub fn value<A1, A2: ?Sized, A3, Out>(
            f: fn(A1, &mut A2, A3) -> Out,
        ) -> value::HkFn<fn(A1, &mut A2, A3) -> Out> {
            value::HkFn(f)
        }
        impl<A1, A2: ?Sized, A3, Out> crate::callback::Callable<(A1, &mut A2, A3)>
            for value::HkFn<fn(A1, &mut A2, A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (A1, &mut A2, A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1, A2: ?Sized, A3, Out> PartialEq for value::HkFn<fn(A1, &mut A2, A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1, A2: ?Sized, A3, Out> crate::callback::IsCallable
            for value::HkFn<fn(A1, &mut A2, A3) -> Out>
        {
        }

        pub mod value {
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);
        }
        impl<A1, A2: ?Sized, A3, Out> crate::callback::CallableWithFixedArguments
            for value::HkFn<fn(A1, &mut A2, A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::Value<A1>,
                crate::callback::argument::ByMut<A2>,
                crate::callback::argument::Value<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1, A2: ?Sized, A3, Out, Arg> crate::callback::Callable<(A1, &mut A2)>
            for crate::callback::argument::LastArgumentProvided<
                value::HkFn<fn(A1, &mut A2, A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::value<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (A1, &mut A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#ref<A1, A2: ?Sized, A3: ?Sized, Out>(
            f: fn(A1, &mut A2, &A3) -> Out,
        ) -> HkFn<fn(A1, &mut A2, &A3) -> Out> {
            HkFn(f)
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::callback::Callable<(A1, &mut A2, &A3)>
            for HkFn<fn(A1, &mut A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (A1, &mut A2, &A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(A1, &mut A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::callback::IsCallable
            for HkFn<fn(A1, &mut A2, &A3) -> Out>
        {
        }

        pub mod r#ref {
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1, A2: ?Sized, A3, Out>(
                    f: fn(A1, &mut A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::callback::argument::LastArgumentProvided<
                    HkFn<fn(A1, &mut A2, &A3) -> Out>,
                    crate::callback::argument::Refed<A3>,
                > {
                    use crate::callback::CallableWithFixedArguments;
                    super::super::r#ref(f)
                        .provide_last_argument(crate::callback::argument::Refed(arg))
                }
            }
            pub use provide_last_argument::provide_last_argument;
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(A1, &mut A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::Value<A1>,
                crate::callback::argument::ByMut<A2>,
                crate::callback::argument::ByRef<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1, A2: ?Sized, A3: ?Sized, Out, Arg> crate::callback::Callable<(A1, &mut A2)>
            for crate::callback::argument::LastArgumentProvided<
                HkFn<fn(A1, &mut A2, &A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#ref<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (A1, &mut A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#mut<A1, A2: ?Sized, A3: ?Sized, Out>(
            f: fn(A1, &mut A2, &mut A3) -> Out,
        ) -> HkFn<fn(A1, &mut A2, &mut A3) -> Out> {
            HkFn(f)
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::callback::Callable<(A1, &mut A2, &mut A3)>
            for HkFn<fn(A1, &mut A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (A1, &mut A2, &mut A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(A1, &mut A2, &mut A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::callback::IsCallable
            for HkFn<fn(A1, &mut A2, &mut A3) -> Out>
        {
        }

        pub mod r#mut {}
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(A1, &mut A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::Value<A1>,
                crate::callback::argument::ByMut<A2>,
                crate::callback::argument::ByMut<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1, A2: ?Sized, A3: ?Sized, Out, Arg> crate::callback::Callable<(A1, &mut A2)>
            for crate::callback::argument::LastArgumentProvided<
                HkFn<fn(A1, &mut A2, &mut A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#mut<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (A1, &mut A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
    }
    impl<A1, A2: ?Sized, Out> crate::callback::CallableWithFixedArguments
        for HkFn<fn(A1, &mut A2) -> Out>
    {
        type FixedArgumentTypes = (
            crate::callback::argument::Value<A1>,
            crate::callback::argument::ByMut<A2>,
        );
    }
    #[cfg(aa)]
    impl<A1, A2: ?Sized, Out, Arg> crate::callback::Callable<(A1,)>
        for crate::callback::argument::LastArgumentProvided<HkFn<fn(A1, &mut A2) -> Out>, Arg>
    where
        Arg: crate::callback::argument::ProvideArgument<
            ProvideArgumentType = crate::callback::argument::r#mut<A2>,
        >,
    {
        type Output = Out;
        fn call_fn(&self, (A1,): (A1,)) -> Self::Output {
            self.last_argument.provide_argument_to(|last_argument| {
                crate::callback::Callable::<_>::call_fn(&self.f, (A1, last_argument))
            })
        }
    }
}
impl<A1, Out> crate::callback::CallableWithFixedArguments for fn(A1) -> Out {
    type FixedArgumentTypes = (crate::callback::argument::Value<A1>,);
}
#[cfg(aa)]
impl<A1, Out, Arg> crate::callback::Callable<()>
    for crate::callback::argument::LastArgumentProvided<fn(A1) -> Out, Arg>
where
    Arg: crate::callback::argument::ProvideArgument<
        ProvideArgumentType = crate::callback::argument::value<A1>,
    >,
{
    type Output = Out;
    fn call_fn(&self, (): ()) -> Self::Output {
        self.last_argument.provide_argument_to(|last_argument| {
            crate::callback::Callable::<_>::call_fn(&self.f, (last_argument,))
        })
    }
}
pub fn r#ref<A1: ?Sized, Out>(f: fn(&A1) -> Out) -> HkFn<fn(&A1) -> Out> {
    HkFn(f)
}
impl<A1: ?Sized, Out> crate::callback::Callable<(&A1,)> for HkFn<fn(&A1) -> Out> {
    type Output = Out;
    fn call_fn(&self, (A1,): (&A1,)) -> Self::Output {
        self.0(A1)
    }
}
impl<A1: ?Sized, Out> PartialEq for HkFn<fn(&A1) -> Out> {
    fn eq(&self, other: &Self) -> bool {
        self.0 as usize == other.0 as usize
    }
}
impl<A1: ?Sized, Out> crate::callback::IsCallable for HkFn<fn(&A1) -> Out> {}

pub mod r#ref {
    mod provide_last_argument {
        use super::super::HkFn;
        pub fn provide_last_argument<A1, Out>(
            f: fn(&A1) -> Out,
            arg: A1,
        ) -> crate::callback::argument::LastArgumentProvided<
            HkFn<fn(&A1) -> Out>,
            crate::callback::argument::Refed<A1>,
        > {
            use crate::callback::CallableWithFixedArguments;
            super::super::r#ref(f).provide_last_argument(crate::callback::argument::Refed(arg))
        }
    }
    use super::HkFn;
    pub use provide_last_argument::provide_last_argument;
    pub fn value<A1: ?Sized, A2, Out>(f: fn(&A1, A2) -> Out) -> value::HkFn<fn(&A1, A2) -> Out> {
        value::HkFn(f)
    }
    impl<A1: ?Sized, A2, Out> crate::callback::Callable<(&A1, A2)> for value::HkFn<fn(&A1, A2) -> Out> {
        type Output = Out;
        fn call_fn(&self, (A1, A2): (&A1, A2)) -> Self::Output {
            self.0(A1, A2)
        }
    }
    impl<A1: ?Sized, A2, Out> PartialEq for value::HkFn<fn(&A1, A2) -> Out> {
        fn eq(&self, other: &Self) -> bool {
            self.0 as usize == other.0 as usize
        }
    }
    impl<A1: ?Sized, A2, Out> crate::callback::IsCallable for value::HkFn<fn(&A1, A2) -> Out> {}

    pub mod value {
        #[derive(Debug, Clone, Copy)]
        pub struct HkFn<F>(pub(super) F);

        pub fn value<A1: ?Sized, A2, A3, Out>(
            f: fn(&A1, A2, A3) -> Out,
        ) -> value::HkFn<fn(&A1, A2, A3) -> Out> {
            value::HkFn(f)
        }
        impl<A1: ?Sized, A2, A3, Out> crate::callback::Callable<(&A1, A2, A3)>
            for value::HkFn<fn(&A1, A2, A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&A1, A2, A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2, A3, Out> PartialEq for value::HkFn<fn(&A1, A2, A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2, A3, Out> crate::callback::IsCallable for value::HkFn<fn(&A1, A2, A3) -> Out> {}

        pub mod value {
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);
        }
        impl<A1: ?Sized, A2, A3, Out> crate::callback::CallableWithFixedArguments
            for value::HkFn<fn(&A1, A2, A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByRef<A1>,
                crate::callback::argument::Value<A2>,
                crate::callback::argument::Value<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2, A3, Out, Arg> crate::callback::Callable<(&A1, A2)>
            for crate::callback::argument::LastArgumentProvided<
                value::HkFn<fn(&A1, A2, A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::value<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&A1, A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#ref<A1: ?Sized, A2, A3: ?Sized, Out>(
            f: fn(&A1, A2, &A3) -> Out,
        ) -> HkFn<fn(&A1, A2, &A3) -> Out> {
            HkFn(f)
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::callback::Callable<(&A1, A2, &A3)>
            for HkFn<fn(&A1, A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&A1, A2, &A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> PartialEq for HkFn<fn(&A1, A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::callback::IsCallable
            for HkFn<fn(&A1, A2, &A3) -> Out>
        {
        }

        pub mod r#ref {
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1: ?Sized, A2, A3, Out>(
                    f: fn(&A1, A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::callback::argument::LastArgumentProvided<
                    HkFn<fn(&A1, A2, &A3) -> Out>,
                    crate::callback::argument::Refed<A3>,
                > {
                    use crate::callback::CallableWithFixedArguments;
                    super::super::r#ref(f)
                        .provide_last_argument(crate::callback::argument::Refed(arg))
                }
            }
            pub use provide_last_argument::provide_last_argument;
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(&A1, A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByRef<A1>,
                crate::callback::argument::Value<A2>,
                crate::callback::argument::ByRef<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2, A3: ?Sized, Out, Arg> crate::callback::Callable<(&A1, A2)>
            for crate::callback::argument::LastArgumentProvided<HkFn<fn(&A1, A2, &A3) -> Out>, Arg>
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#ref<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&A1, A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#mut<A1: ?Sized, A2, A3: ?Sized, Out>(
            f: fn(&A1, A2, &mut A3) -> Out,
        ) -> HkFn<fn(&A1, A2, &mut A3) -> Out> {
            HkFn(f)
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::callback::Callable<(&A1, A2, &mut A3)>
            for HkFn<fn(&A1, A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&A1, A2, &mut A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> PartialEq for HkFn<fn(&A1, A2, &mut A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::callback::IsCallable
            for HkFn<fn(&A1, A2, &mut A3) -> Out>
        {
        }

        pub mod r#mut {}
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(&A1, A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByRef<A1>,
                crate::callback::argument::Value<A2>,
                crate::callback::argument::ByMut<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2, A3: ?Sized, Out, Arg> crate::callback::Callable<(&A1, A2)>
            for crate::callback::argument::LastArgumentProvided<
                HkFn<fn(&A1, A2, &mut A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#mut<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&A1, A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
    }
    impl<A1: ?Sized, A2, Out> crate::callback::CallableWithFixedArguments
        for value::HkFn<fn(&A1, A2) -> Out>
    {
        type FixedArgumentTypes = (
            crate::callback::argument::ByRef<A1>,
            crate::callback::argument::Value<A2>,
        );
    }
    #[cfg(aa)]
    impl<A1: ?Sized, A2, Out, Arg> crate::callback::Callable<(&A1,)>
        for crate::callback::argument::LastArgumentProvided<value::HkFn<fn(&A1, A2) -> Out>, Arg>
    where
        Arg: crate::callback::argument::ProvideArgument<
            ProvideArgumentType = crate::callback::argument::value<A2>,
        >,
    {
        type Output = Out;
        fn call_fn(&self, (A1,): (&A1,)) -> Self::Output {
            self.last_argument.provide_argument_to(|last_argument| {
                crate::callback::Callable::<_>::call_fn(&self.f, (A1, last_argument))
            })
        }
    }
    pub fn r#ref<A1: ?Sized, A2: ?Sized, Out>(f: fn(&A1, &A2) -> Out) -> HkFn<fn(&A1, &A2) -> Out> {
        HkFn(f)
    }
    impl<A1: ?Sized, A2: ?Sized, Out> crate::callback::Callable<(&A1, &A2)>
        for HkFn<fn(&A1, &A2) -> Out>
    {
        type Output = Out;
        fn call_fn(&self, (A1, A2): (&A1, &A2)) -> Self::Output {
            self.0(A1, A2)
        }
    }
    impl<A1: ?Sized, A2: ?Sized, Out> PartialEq for HkFn<fn(&A1, &A2) -> Out> {
        fn eq(&self, other: &Self) -> bool {
            self.0 as usize == other.0 as usize
        }
    }
    impl<A1: ?Sized, A2: ?Sized, Out> crate::callback::IsCallable for HkFn<fn(&A1, &A2) -> Out> {}

    pub mod r#ref {
        mod provide_last_argument {
            use super::super::HkFn;
            pub fn provide_last_argument<A1: ?Sized, A2, Out>(
                f: fn(&A1, &A2) -> Out,
                arg: A2,
            ) -> crate::callback::argument::LastArgumentProvided<
                HkFn<fn(&A1, &A2) -> Out>,
                crate::callback::argument::Refed<A2>,
            > {
                use crate::callback::CallableWithFixedArguments;
                super::super::r#ref(f).provide_last_argument(crate::callback::argument::Refed(arg))
            }
        }
        use super::HkFn;
        pub use provide_last_argument::provide_last_argument;
        pub fn value<A1: ?Sized, A2: ?Sized, A3, Out>(
            f: fn(&A1, &A2, A3) -> Out,
        ) -> value::HkFn<fn(&A1, &A2, A3) -> Out> {
            value::HkFn(f)
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::callback::Callable<(&A1, &A2, A3)>
            for value::HkFn<fn(&A1, &A2, A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&A1, &A2, A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> PartialEq for value::HkFn<fn(&A1, &A2, A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::callback::IsCallable
            for value::HkFn<fn(&A1, &A2, A3) -> Out>
        {
        }

        pub mod value {
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::callback::CallableWithFixedArguments
            for value::HkFn<fn(&A1, &A2, A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByRef<A1>,
                crate::callback::argument::ByRef<A2>,
                crate::callback::argument::Value<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2: ?Sized, A3, Out, Arg> crate::callback::Callable<(&A1, &A2)>
            for crate::callback::argument::LastArgumentProvided<
                value::HkFn<fn(&A1, &A2, A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::value<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&A1, &A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#ref<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>(
            f: fn(&A1, &A2, &A3) -> Out,
        ) -> HkFn<fn(&A1, &A2, &A3) -> Out> {
            HkFn(f)
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::Callable<(&A1, &A2, &A3)>
            for HkFn<fn(&A1, &A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&A1, &A2, &A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(&A1, &A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::IsCallable
            for HkFn<fn(&A1, &A2, &A3) -> Out>
        {
        }

        pub mod r#ref {
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3, Out>(
                    f: fn(&A1, &A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::callback::argument::LastArgumentProvided<
                    HkFn<fn(&A1, &A2, &A3) -> Out>,
                    crate::callback::argument::Refed<A3>,
                > {
                    use crate::callback::CallableWithFixedArguments;
                    super::super::r#ref(f)
                        .provide_last_argument(crate::callback::argument::Refed(arg))
                }
            }
            pub use provide_last_argument::provide_last_argument;
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(&A1, &A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByRef<A1>,
                crate::callback::argument::ByRef<A2>,
                crate::callback::argument::ByRef<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out, Arg> crate::callback::Callable<(&A1, &A2)>
            for crate::callback::argument::LastArgumentProvided<HkFn<fn(&A1, &A2, &A3) -> Out>, Arg>
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#ref<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&A1, &A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#mut<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>(
            f: fn(&A1, &A2, &mut A3) -> Out,
        ) -> HkFn<fn(&A1, &A2, &mut A3) -> Out> {
            HkFn(f)
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::Callable<(&A1, &A2, &mut A3)>
            for HkFn<fn(&A1, &A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&A1, &A2, &mut A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(&A1, &A2, &mut A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::IsCallable
            for HkFn<fn(&A1, &A2, &mut A3) -> Out>
        {
        }

        pub mod r#mut {}
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(&A1, &A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByRef<A1>,
                crate::callback::argument::ByRef<A2>,
                crate::callback::argument::ByMut<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out, Arg> crate::callback::Callable<(&A1, &A2)>
            for crate::callback::argument::LastArgumentProvided<
                HkFn<fn(&A1, &A2, &mut A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#mut<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&A1, &A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
    }
    impl<A1: ?Sized, A2: ?Sized, Out> crate::callback::CallableWithFixedArguments
        for HkFn<fn(&A1, &A2) -> Out>
    {
        type FixedArgumentTypes = (
            crate::callback::argument::ByRef<A1>,
            crate::callback::argument::ByRef<A2>,
        );
    }
    #[cfg(aa)]
    impl<A1: ?Sized, A2: ?Sized, Out, Arg> crate::callback::Callable<(&A1,)>
        for crate::callback::argument::LastArgumentProvided<HkFn<fn(&A1, &A2) -> Out>, Arg>
    where
        Arg: crate::callback::argument::ProvideArgument<
            ProvideArgumentType = crate::callback::argument::r#ref<A2>,
        >,
    {
        type Output = Out;
        fn call_fn(&self, (A1,): (&A1,)) -> Self::Output {
            self.last_argument.provide_argument_to(|last_argument| {
                crate::callback::Callable::<_>::call_fn(&self.f, (A1, last_argument))
            })
        }
    }
    pub fn r#mut<A1: ?Sized, A2: ?Sized, Out>(
        f: fn(&A1, &mut A2) -> Out,
    ) -> HkFn<fn(&A1, &mut A2) -> Out> {
        HkFn(f)
    }
    impl<A1: ?Sized, A2: ?Sized, Out> crate::callback::Callable<(&A1, &mut A2)>
        for HkFn<fn(&A1, &mut A2) -> Out>
    {
        type Output = Out;
        fn call_fn(&self, (A1, A2): (&A1, &mut A2)) -> Self::Output {
            self.0(A1, A2)
        }
    }
    impl<A1: ?Sized, A2: ?Sized, Out> PartialEq for HkFn<fn(&A1, &mut A2) -> Out> {
        fn eq(&self, other: &Self) -> bool {
            self.0 as usize == other.0 as usize
        }
    }
    impl<A1: ?Sized, A2: ?Sized, Out> crate::callback::IsCallable for HkFn<fn(&A1, &mut A2) -> Out> {}

    pub mod r#mut {
        use super::HkFn;
        pub fn value<A1: ?Sized, A2: ?Sized, A3, Out>(
            f: fn(&A1, &mut A2, A3) -> Out,
        ) -> value::HkFn<fn(&A1, &mut A2, A3) -> Out> {
            value::HkFn(f)
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::callback::Callable<(&A1, &mut A2, A3)>
            for value::HkFn<fn(&A1, &mut A2, A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&A1, &mut A2, A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> PartialEq for value::HkFn<fn(&A1, &mut A2, A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::callback::IsCallable
            for value::HkFn<fn(&A1, &mut A2, A3) -> Out>
        {
        }

        pub mod value {
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::callback::CallableWithFixedArguments
            for value::HkFn<fn(&A1, &mut A2, A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByRef<A1>,
                crate::callback::argument::ByMut<A2>,
                crate::callback::argument::Value<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2: ?Sized, A3, Out, Arg> crate::callback::Callable<(&A1, &mut A2)>
            for crate::callback::argument::LastArgumentProvided<
                value::HkFn<fn(&A1, &mut A2, A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::value<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&A1, &mut A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#ref<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>(
            f: fn(&A1, &mut A2, &A3) -> Out,
        ) -> HkFn<fn(&A1, &mut A2, &A3) -> Out> {
            HkFn(f)
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::Callable<(&A1, &mut A2, &A3)>
            for HkFn<fn(&A1, &mut A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&A1, &mut A2, &A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(&A1, &mut A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::IsCallable
            for HkFn<fn(&A1, &mut A2, &A3) -> Out>
        {
        }

        pub mod r#ref {
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3, Out>(
                    f: fn(&A1, &mut A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::callback::argument::LastArgumentProvided<
                    HkFn<fn(&A1, &mut A2, &A3) -> Out>,
                    crate::callback::argument::Refed<A3>,
                > {
                    use crate::callback::CallableWithFixedArguments;
                    super::super::r#ref(f)
                        .provide_last_argument(crate::callback::argument::Refed(arg))
                }
            }
            pub use provide_last_argument::provide_last_argument;
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(&A1, &mut A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByRef<A1>,
                crate::callback::argument::ByMut<A2>,
                crate::callback::argument::ByRef<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out, Arg> crate::callback::Callable<(&A1, &mut A2)>
            for crate::callback::argument::LastArgumentProvided<
                HkFn<fn(&A1, &mut A2, &A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#ref<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&A1, &mut A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#mut<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>(
            f: fn(&A1, &mut A2, &mut A3) -> Out,
        ) -> HkFn<fn(&A1, &mut A2, &mut A3) -> Out> {
            HkFn(f)
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>
            crate::callback::Callable<(&A1, &mut A2, &mut A3)>
            for HkFn<fn(&A1, &mut A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&A1, &mut A2, &mut A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(&A1, &mut A2, &mut A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::IsCallable
            for HkFn<fn(&A1, &mut A2, &mut A3) -> Out>
        {
        }

        pub mod r#mut {}
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(&A1, &mut A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByRef<A1>,
                crate::callback::argument::ByMut<A2>,
                crate::callback::argument::ByMut<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out, Arg> crate::callback::Callable<(&A1, &mut A2)>
            for crate::callback::argument::LastArgumentProvided<
                HkFn<fn(&A1, &mut A2, &mut A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#mut<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&A1, &mut A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
    }
    impl<A1: ?Sized, A2: ?Sized, Out> crate::callback::CallableWithFixedArguments
        for HkFn<fn(&A1, &mut A2) -> Out>
    {
        type FixedArgumentTypes = (
            crate::callback::argument::ByRef<A1>,
            crate::callback::argument::ByMut<A2>,
        );
    }
    #[cfg(aa)]
    impl<A1: ?Sized, A2: ?Sized, Out, Arg> crate::callback::Callable<(&A1,)>
        for crate::callback::argument::LastArgumentProvided<HkFn<fn(&A1, &mut A2) -> Out>, Arg>
    where
        Arg: crate::callback::argument::ProvideArgument<
            ProvideArgumentType = crate::callback::argument::r#mut<A2>,
        >,
    {
        type Output = Out;
        fn call_fn(&self, (A1,): (&A1,)) -> Self::Output {
            self.last_argument.provide_argument_to(|last_argument| {
                crate::callback::Callable::<_>::call_fn(&self.f, (A1, last_argument))
            })
        }
    }
}
impl<A1: ?Sized, Out> crate::callback::CallableWithFixedArguments for HkFn<fn(&A1) -> Out> {
    type FixedArgumentTypes = (crate::callback::argument::ByRef<A1>,);
}
#[cfg(aa)]
impl<A1: ?Sized, Out, Arg> crate::callback::Callable<()>
    for crate::callback::argument::LastArgumentProvided<HkFn<fn(&A1) -> Out>, Arg>
where
    Arg: crate::callback::argument::ProvideArgument<
        ProvideArgumentType = crate::callback::argument::r#ref<A1>,
    >,
{
    type Output = Out;
    fn call_fn(&self, (): ()) -> Self::Output {
        self.last_argument.provide_argument_to(|last_argument| {
            crate::callback::Callable::<_>::call_fn(&self.f, (last_argument,))
        })
    }
}
pub fn r#mut<A1: ?Sized, Out>(f: fn(&mut A1) -> Out) -> HkFn<fn(&mut A1) -> Out> {
    HkFn(f)
}
impl<A1: ?Sized, Out> crate::callback::Callable<(&mut A1,)> for HkFn<fn(&mut A1) -> Out> {
    type Output = Out;
    fn call_fn(&self, (A1,): (&mut A1,)) -> Self::Output {
        self.0(A1)
    }
}
impl<A1: ?Sized, Out> PartialEq for HkFn<fn(&mut A1) -> Out> {
    fn eq(&self, other: &Self) -> bool {
        self.0 as usize == other.0 as usize
    }
}
impl<A1: ?Sized, Out> crate::callback::IsCallable for HkFn<fn(&mut A1) -> Out> {}

pub mod r#mut {
    use super::HkFn;
    pub fn value<A1: ?Sized, A2, Out>(
        f: fn(&mut A1, A2) -> Out,
    ) -> value::HkFn<fn(&mut A1, A2) -> Out> {
        value::HkFn(f)
    }
    impl<A1: ?Sized, A2, Out> crate::callback::Callable<(&mut A1, A2)>
        for value::HkFn<fn(&mut A1, A2) -> Out>
    {
        type Output = Out;
        fn call_fn(&self, (A1, A2): (&mut A1, A2)) -> Self::Output {
            self.0(A1, A2)
        }
    }
    impl<A1: ?Sized, A2, Out> PartialEq for value::HkFn<fn(&mut A1, A2) -> Out> {
        fn eq(&self, other: &Self) -> bool {
            self.0 as usize == other.0 as usize
        }
    }
    impl<A1: ?Sized, A2, Out> crate::callback::IsCallable for value::HkFn<fn(&mut A1, A2) -> Out> {}

    pub mod value {
        #[derive(Debug, Clone, Copy)]
        pub struct HkFn<F>(pub(super) F);

        pub fn value<A1: ?Sized, A2, A3, Out>(
            f: fn(&mut A1, A2, A3) -> Out,
        ) -> value::HkFn<fn(&mut A1, A2, A3) -> Out> {
            value::HkFn(f)
        }
        impl<A1: ?Sized, A2, A3, Out> crate::callback::Callable<(&mut A1, A2, A3)>
            for value::HkFn<fn(&mut A1, A2, A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&mut A1, A2, A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2, A3, Out> PartialEq for value::HkFn<fn(&mut A1, A2, A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2, A3, Out> crate::callback::IsCallable
            for value::HkFn<fn(&mut A1, A2, A3) -> Out>
        {
        }

        pub mod value {
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);
        }
        impl<A1: ?Sized, A2, A3, Out> crate::callback::CallableWithFixedArguments
            for value::HkFn<fn(&mut A1, A2, A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByMut<A1>,
                crate::callback::argument::Value<A2>,
                crate::callback::argument::Value<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2, A3, Out, Arg> crate::callback::Callable<(&mut A1, A2)>
            for crate::callback::argument::LastArgumentProvided<
                value::HkFn<fn(&mut A1, A2, A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::value<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&mut A1, A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#ref<A1: ?Sized, A2, A3: ?Sized, Out>(
            f: fn(&mut A1, A2, &A3) -> Out,
        ) -> HkFn<fn(&mut A1, A2, &A3) -> Out> {
            HkFn(f)
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::callback::Callable<(&mut A1, A2, &A3)>
            for HkFn<fn(&mut A1, A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&mut A1, A2, &A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> PartialEq for HkFn<fn(&mut A1, A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::callback::IsCallable
            for HkFn<fn(&mut A1, A2, &A3) -> Out>
        {
        }

        pub mod r#ref {
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1: ?Sized, A2, A3, Out>(
                    f: fn(&mut A1, A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::callback::argument::LastArgumentProvided<
                    HkFn<fn(&mut A1, A2, &A3) -> Out>,
                    crate::callback::argument::Refed<A3>,
                > {
                    use crate::callback::CallableWithFixedArguments;
                    super::super::r#ref(f)
                        .provide_last_argument(crate::callback::argument::Refed(arg))
                }
            }
            pub use provide_last_argument::provide_last_argument;
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(&mut A1, A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByMut<A1>,
                crate::callback::argument::Value<A2>,
                crate::callback::argument::ByRef<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2, A3: ?Sized, Out, Arg> crate::callback::Callable<(&mut A1, A2)>
            for crate::callback::argument::LastArgumentProvided<
                HkFn<fn(&mut A1, A2, &A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#ref<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&mut A1, A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#mut<A1: ?Sized, A2, A3: ?Sized, Out>(
            f: fn(&mut A1, A2, &mut A3) -> Out,
        ) -> HkFn<fn(&mut A1, A2, &mut A3) -> Out> {
            HkFn(f)
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::callback::Callable<(&mut A1, A2, &mut A3)>
            for HkFn<fn(&mut A1, A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&mut A1, A2, &mut A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> PartialEq for HkFn<fn(&mut A1, A2, &mut A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::callback::IsCallable
            for HkFn<fn(&mut A1, A2, &mut A3) -> Out>
        {
        }

        pub mod r#mut {}
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(&mut A1, A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByMut<A1>,
                crate::callback::argument::Value<A2>,
                crate::callback::argument::ByMut<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2, A3: ?Sized, Out, Arg> crate::callback::Callable<(&mut A1, A2)>
            for crate::callback::argument::LastArgumentProvided<
                HkFn<fn(&mut A1, A2, &mut A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#mut<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&mut A1, A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
    }
    impl<A1: ?Sized, A2, Out> crate::callback::CallableWithFixedArguments
        for value::HkFn<fn(&mut A1, A2) -> Out>
    {
        type FixedArgumentTypes = (
            crate::callback::argument::ByMut<A1>,
            crate::callback::argument::Value<A2>,
        );
    }
    #[cfg(aa)]
    impl<A1: ?Sized, A2, Out, Arg> crate::callback::Callable<(&mut A1,)>
        for crate::callback::argument::LastArgumentProvided<
            value::HkFn<fn(&mut A1, A2) -> Out>,
            Arg,
        >
    where
        Arg: crate::callback::argument::ProvideArgument<
            ProvideArgumentType = crate::callback::argument::value<A2>,
        >,
    {
        type Output = Out;
        fn call_fn(&self, (A1,): (&mut A1,)) -> Self::Output {
            self.last_argument.provide_argument_to(|last_argument| {
                crate::callback::Callable::<_>::call_fn(&self.f, (A1, last_argument))
            })
        }
    }
    pub fn r#ref<A1: ?Sized, A2: ?Sized, Out>(
        f: fn(&mut A1, &A2) -> Out,
    ) -> HkFn<fn(&mut A1, &A2) -> Out> {
        HkFn(f)
    }
    impl<A1: ?Sized, A2: ?Sized, Out> crate::callback::Callable<(&mut A1, &A2)>
        for HkFn<fn(&mut A1, &A2) -> Out>
    {
        type Output = Out;
        fn call_fn(&self, (A1, A2): (&mut A1, &A2)) -> Self::Output {
            self.0(A1, A2)
        }
    }
    impl<A1: ?Sized, A2: ?Sized, Out> PartialEq for HkFn<fn(&mut A1, &A2) -> Out> {
        fn eq(&self, other: &Self) -> bool {
            self.0 as usize == other.0 as usize
        }
    }
    impl<A1: ?Sized, A2: ?Sized, Out> crate::callback::IsCallable for HkFn<fn(&mut A1, &A2) -> Out> {}

    pub mod r#ref {
        mod provide_last_argument {
            use super::super::HkFn;
            pub fn provide_last_argument<A1: ?Sized, A2, Out>(
                f: fn(&mut A1, &A2) -> Out,
                arg: A2,
            ) -> crate::callback::argument::LastArgumentProvided<
                HkFn<fn(&mut A1, &A2) -> Out>,
                crate::callback::argument::Refed<A2>,
            > {
                use crate::callback::CallableWithFixedArguments;
                super::super::r#ref(f).provide_last_argument(crate::callback::argument::Refed(arg))
            }
        }
        use super::HkFn;
        pub use provide_last_argument::provide_last_argument;
        pub fn value<A1: ?Sized, A2: ?Sized, A3, Out>(
            f: fn(&mut A1, &A2, A3) -> Out,
        ) -> value::HkFn<fn(&mut A1, &A2, A3) -> Out> {
            value::HkFn(f)
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::callback::Callable<(&mut A1, &A2, A3)>
            for value::HkFn<fn(&mut A1, &A2, A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&mut A1, &A2, A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> PartialEq for value::HkFn<fn(&mut A1, &A2, A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::callback::IsCallable
            for value::HkFn<fn(&mut A1, &A2, A3) -> Out>
        {
        }

        pub mod value {
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::callback::CallableWithFixedArguments
            for value::HkFn<fn(&mut A1, &A2, A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByMut<A1>,
                crate::callback::argument::ByRef<A2>,
                crate::callback::argument::Value<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2: ?Sized, A3, Out, Arg> crate::callback::Callable<(&mut A1, &A2)>
            for crate::callback::argument::LastArgumentProvided<
                value::HkFn<fn(&mut A1, &A2, A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::value<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&mut A1, &A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#ref<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>(
            f: fn(&mut A1, &A2, &A3) -> Out,
        ) -> HkFn<fn(&mut A1, &A2, &A3) -> Out> {
            HkFn(f)
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::Callable<(&mut A1, &A2, &A3)>
            for HkFn<fn(&mut A1, &A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&mut A1, &A2, &A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(&mut A1, &A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::IsCallable
            for HkFn<fn(&mut A1, &A2, &A3) -> Out>
        {
        }

        pub mod r#ref {
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3, Out>(
                    f: fn(&mut A1, &A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::callback::argument::LastArgumentProvided<
                    HkFn<fn(&mut A1, &A2, &A3) -> Out>,
                    crate::callback::argument::Refed<A3>,
                > {
                    use crate::callback::CallableWithFixedArguments;
                    super::super::r#ref(f)
                        .provide_last_argument(crate::callback::argument::Refed(arg))
                }
            }
            pub use provide_last_argument::provide_last_argument;
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(&mut A1, &A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByMut<A1>,
                crate::callback::argument::ByRef<A2>,
                crate::callback::argument::ByRef<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out, Arg> crate::callback::Callable<(&mut A1, &A2)>
            for crate::callback::argument::LastArgumentProvided<
                HkFn<fn(&mut A1, &A2, &A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#ref<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&mut A1, &A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#mut<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>(
            f: fn(&mut A1, &A2, &mut A3) -> Out,
        ) -> HkFn<fn(&mut A1, &A2, &mut A3) -> Out> {
            HkFn(f)
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>
            crate::callback::Callable<(&mut A1, &A2, &mut A3)>
            for HkFn<fn(&mut A1, &A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&mut A1, &A2, &mut A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(&mut A1, &A2, &mut A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::IsCallable
            for HkFn<fn(&mut A1, &A2, &mut A3) -> Out>
        {
        }

        pub mod r#mut {}
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(&mut A1, &A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByMut<A1>,
                crate::callback::argument::ByRef<A2>,
                crate::callback::argument::ByMut<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out, Arg> crate::callback::Callable<(&mut A1, &A2)>
            for crate::callback::argument::LastArgumentProvided<
                HkFn<fn(&mut A1, &A2, &mut A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#mut<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&mut A1, &A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
    }
    impl<A1: ?Sized, A2: ?Sized, Out> crate::callback::CallableWithFixedArguments
        for HkFn<fn(&mut A1, &A2) -> Out>
    {
        type FixedArgumentTypes = (
            crate::callback::argument::ByMut<A1>,
            crate::callback::argument::ByRef<A2>,
        );
    }
    #[cfg(aa)]
    impl<A1: ?Sized, A2: ?Sized, Out, Arg> crate::callback::Callable<(&mut A1,)>
        for crate::callback::argument::LastArgumentProvided<HkFn<fn(&mut A1, &A2) -> Out>, Arg>
    where
        Arg: crate::callback::argument::ProvideArgument<
            ProvideArgumentType = crate::callback::argument::r#ref<A2>,
        >,
    {
        type Output = Out;
        fn call_fn(&self, (A1,): (&mut A1,)) -> Self::Output {
            self.last_argument.provide_argument_to(|last_argument| {
                crate::callback::Callable::<_>::call_fn(&self.f, (A1, last_argument))
            })
        }
    }
    pub fn r#mut<A1: ?Sized, A2: ?Sized, Out>(
        f: fn(&mut A1, &mut A2) -> Out,
    ) -> HkFn<fn(&mut A1, &mut A2) -> Out> {
        HkFn(f)
    }
    impl<A1: ?Sized, A2: ?Sized, Out> crate::callback::Callable<(&mut A1, &mut A2)>
        for HkFn<fn(&mut A1, &mut A2) -> Out>
    {
        type Output = Out;
        fn call_fn(&self, (A1, A2): (&mut A1, &mut A2)) -> Self::Output {
            self.0(A1, A2)
        }
    }
    impl<A1: ?Sized, A2: ?Sized, Out> PartialEq for HkFn<fn(&mut A1, &mut A2) -> Out> {
        fn eq(&self, other: &Self) -> bool {
            self.0 as usize == other.0 as usize
        }
    }
    impl<A1: ?Sized, A2: ?Sized, Out> crate::callback::IsCallable
        for HkFn<fn(&mut A1, &mut A2) -> Out>
    {
    }

    pub mod r#mut {
        use super::HkFn;
        pub fn value<A1: ?Sized, A2: ?Sized, A3, Out>(
            f: fn(&mut A1, &mut A2, A3) -> Out,
        ) -> value::HkFn<fn(&mut A1, &mut A2, A3) -> Out> {
            value::HkFn(f)
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::callback::Callable<(&mut A1, &mut A2, A3)>
            for value::HkFn<fn(&mut A1, &mut A2, A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&mut A1, &mut A2, A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> PartialEq for value::HkFn<fn(&mut A1, &mut A2, A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::callback::IsCallable
            for value::HkFn<fn(&mut A1, &mut A2, A3) -> Out>
        {
        }

        pub mod value {
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::callback::CallableWithFixedArguments
            for value::HkFn<fn(&mut A1, &mut A2, A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByMut<A1>,
                crate::callback::argument::ByMut<A2>,
                crate::callback::argument::Value<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2: ?Sized, A3, Out, Arg> crate::callback::Callable<(&mut A1, &mut A2)>
            for crate::callback::argument::LastArgumentProvided<
                value::HkFn<fn(&mut A1, &mut A2, A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::value<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&mut A1, &mut A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#ref<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>(
            f: fn(&mut A1, &mut A2, &A3) -> Out,
        ) -> HkFn<fn(&mut A1, &mut A2, &A3) -> Out> {
            HkFn(f)
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>
            crate::callback::Callable<(&mut A1, &mut A2, &A3)>
            for HkFn<fn(&mut A1, &mut A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&mut A1, &mut A2, &A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(&mut A1, &mut A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::IsCallable
            for HkFn<fn(&mut A1, &mut A2, &A3) -> Out>
        {
        }

        pub mod r#ref {
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3, Out>(
                    f: fn(&mut A1, &mut A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::callback::argument::LastArgumentProvided<
                    HkFn<fn(&mut A1, &mut A2, &A3) -> Out>,
                    crate::callback::argument::Refed<A3>,
                > {
                    use crate::callback::CallableWithFixedArguments;
                    super::super::r#ref(f)
                        .provide_last_argument(crate::callback::argument::Refed(arg))
                }
            }
            pub use provide_last_argument::provide_last_argument;
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(&mut A1, &mut A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByMut<A1>,
                crate::callback::argument::ByMut<A2>,
                crate::callback::argument::ByRef<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out, Arg>
            crate::callback::Callable<(&mut A1, &mut A2)>
            for crate::callback::argument::LastArgumentProvided<
                HkFn<fn(&mut A1, &mut A2, &A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#ref<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&mut A1, &mut A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
        pub fn r#mut<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>(
            f: fn(&mut A1, &mut A2, &mut A3) -> Out,
        ) -> HkFn<fn(&mut A1, &mut A2, &mut A3) -> Out> {
            HkFn(f)
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>
            crate::callback::Callable<(&mut A1, &mut A2, &mut A3)>
            for HkFn<fn(&mut A1, &mut A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2, A3): (&mut A1, &mut A2, &mut A3)) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> PartialEq
            for HkFn<fn(&mut A1, &mut A2, &mut A3) -> Out>
        {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::IsCallable
            for HkFn<fn(&mut A1, &mut A2, &mut A3) -> Out>
        {
        }

        pub mod r#mut {}
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::callback::CallableWithFixedArguments
            for HkFn<fn(&mut A1, &mut A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::callback::argument::ByMut<A1>,
                crate::callback::argument::ByMut<A2>,
                crate::callback::argument::ByMut<A3>,
            );
        }
        #[cfg(aa)]
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out, Arg>
            crate::callback::Callable<(&mut A1, &mut A2)>
            for crate::callback::argument::LastArgumentProvided<
                HkFn<fn(&mut A1, &mut A2, &mut A3) -> Out>,
                Arg,
            >
        where
            Arg: crate::callback::argument::ProvideArgument<
                ProvideArgumentType = crate::callback::argument::r#mut<A3>,
            >,
        {
            type Output = Out;
            fn call_fn(&self, (A1, A2): (&mut A1, &mut A2)) -> Self::Output {
                self.last_argument.provide_argument_to(|last_argument| {
                    crate::callback::Callable::<_>::call_fn(&self.f, (A1, A2, last_argument))
                })
            }
        }
    }
    impl<A1: ?Sized, A2: ?Sized, Out> crate::callback::CallableWithFixedArguments
        for HkFn<fn(&mut A1, &mut A2) -> Out>
    {
        type FixedArgumentTypes = (
            crate::callback::argument::ByMut<A1>,
            crate::callback::argument::ByMut<A2>,
        );
    }
    #[cfg(aa)]
    impl<A1: ?Sized, A2: ?Sized, Out, Arg> crate::callback::Callable<(&mut A1,)>
        for crate::callback::argument::LastArgumentProvided<HkFn<fn(&mut A1, &mut A2) -> Out>, Arg>
    where
        Arg: crate::callback::argument::ProvideArgument<
            ProvideArgumentType = crate::callback::argument::r#mut<A2>,
        >,
    {
        type Output = Out;
        fn call_fn(&self, (A1,): (&mut A1,)) -> Self::Output {
            self.last_argument.provide_argument_to(|last_argument| {
                crate::callback::Callable::<_>::call_fn(&self.f, (A1, last_argument))
            })
        }
    }
}
impl<A1: ?Sized, Out> crate::callback::CallableWithFixedArguments for HkFn<fn(&mut A1) -> Out> {
    type FixedArgumentTypes = (crate::callback::argument::ByMut<A1>,);
}
#[cfg(aa)]
impl<A1: ?Sized, Out, Arg> crate::callback::Callable<()>
    for crate::callback::argument::LastArgumentProvided<HkFn<fn(&mut A1) -> Out>, Arg>
where
    Arg: crate::callback::argument::ProvideArgument<
        ProvideArgumentType = crate::callback::argument::r#mut<A1>,
    >,
{
    type Output = Out;
    fn call_fn(&self, (): ()) -> Self::Output {
        self.last_argument.provide_argument_to(|last_argument| {
            crate::callback::Callable::<_>::call_fn(&self.f, (last_argument,))
        })
    }
}

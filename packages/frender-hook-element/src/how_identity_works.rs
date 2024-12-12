use std::{marker::PhantomData, pin::Pin};

trait IntoStaticOwned {
    type StaticOwned: 'static;
}

impl IntoStaticOwned for &str {
    type StaticOwned = String;
}

trait FnMut1<Arg>: FnMut(Arg) -> Self::_Output {
    type _Output;
}

impl<F: ?Sized + FnMut(Arg) -> Out, Arg, Out> FnMut1<Arg> for F {
    type _Output = Out;
}

struct Test<HookData, F, SO>(HookData, F, PhantomData<SO>)
where
    F: for<'a> FnMut1<Pin<&'a mut HookData>, _Output: IntoStaticOwned<StaticOwned = SO>>;

fn test() {
    #[inline(always)]
    const fn identity<F: FnMut(Pin<&mut String>) -> &str>(f: F) -> F {
        f
    }

    Test(
        String::new(),
        identity(|s: Pin<&mut String>| s.into_ref().get_ref().as_str()),
        PhantomData::<String>,
    );

    #[cfg(compile_fail)]
    Test(
        String::new(),
        |s: Pin<&mut String>| s.into_ref().get_ref().as_str(),
        PhantomData::<String>,
    );
}

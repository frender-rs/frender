use std::marker::PhantomData;

pub struct FnMutWithRenderContext<U, S>(pub U, pub PhantomData<S>);

pub struct FnMutOutputElement<U>(pub U);

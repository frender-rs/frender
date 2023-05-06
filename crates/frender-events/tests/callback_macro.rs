use frender_events::{
    callback,
    callback::argument::{self, ArgumentTypes},
    callback::{argument::LastArgumentProvided, CallableWithFixedArguments},
    Callable, IsCallable,
};

#[test]
fn provide_borrowed() {}

#[test]
fn infer_provide_last_argument_refed() {
    let _ = callback!(|v: &_| *v).provide_last_argument_refed(1u8);
    let cbk = callback!(|v: &_| *v).provide_last_argument_refed(1u8);
    assert_eq!(cbk.call_fn(()), 1);
}

#[test]
fn infer() {
    fn provide_with_1<
        C: CallableWithFixedArguments<FixedArgumentTypes = (argument::Value<u8>,)>,
    >(
        callable: C,
    ) -> LastArgumentProvided<C, argument::Copied<u8>> {
        callable.provide_last_argument_copied(1)
    }

    let _ = provide_with_1(callback!(|v| v + 1));
    let cbk = provide_with_1(callback!(|v| v + 1));

    assert_eq!(cbk.call_fn(()), 2);

    fn provide_with_2<C: CallableWithFixedArguments>(
        callable: C,
    ) -> LastArgumentProvided<C, argument::Copied<u8>>
    where
        C::FixedArgumentTypes: ArgumentTypes<Last = argument::Value<u8>>,
    {
        callable.provide_last_argument_copied(2)
    }

    let _ = provide_with_2(callback!(|a: u8, b| a + b));
    let cbk = provide_with_2(callback!(|a: u8, b| a + b));

    assert_eq!(cbk.call_fn((3,)), 5);
}

#[test]
fn infer_ref() {
    fn provide_with_ref<
        C: CallableWithFixedArguments<FixedArgumentTypes = (argument::ByRef<u8>,)>,
    >(
        callable: C,
    ) -> LastArgumentProvided<C, argument::Refed<u8>> {
        callable.provide_last_argument_refed(5)
    }

    let _ = provide_with_ref(callback!(|v: &_| Vec::from_iter([*v; 3])));

    let cbk = provide_with_ref(callback!(|v: &_| Vec::from_iter([*v; 3])));

    assert_eq!(cbk.call_fn(()), [5, 5, 5]);

    fn provide_with_ref_args<C: CallableWithFixedArguments>(
        callable: C,
    ) -> LastArgumentProvided<C, argument::Refed<Vec<u8>>>
    where
        C::FixedArgumentTypes: ArgumentTypes<Last = argument::ByRef<Vec<u8>>>,
    {
        callable.provide_last_argument_refed(vec![1, 2, 3])
    }

    let _ = provide_with_ref_args(callback!(|a: u8, list: &Vec<u8>| list
        .iter()
        .map(|&v| v + a)
        .collect::<Vec<_>>()));
    let cbk = provide_with_ref_args(callback!(|a: u8, list: &Vec<u8>| list
        .iter()
        .map(|&v| v + a)
        .collect::<Vec<_>>()));

    assert_eq!(cbk.call_fn((1,)), [2, 3, 4]);
}

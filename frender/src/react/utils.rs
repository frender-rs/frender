pub fn fn_once_in_runtime<R, F: FnOnce() -> R>(func: F) -> impl FnMut() -> R {
    let mut func = Some(func);

    move || -> R {
        let func = func.take().expect("fn is expected to only invoked once");
        func()
    }
}

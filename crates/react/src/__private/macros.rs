#[macro_export]
macro_rules! __impl_pass_dep {
    ($dep:ident) => {
        $crate::auto_wrap_rc!($dep)
    };
    ($dep:ident = $dep_expr:expr) => {
        $crate::auto_wrap_rc!($dep_expr)
    };
}

#[macro_export]
macro_rules! __impl_let_dep_list {
    ({ $dep_tuple:expr } $dep:ident) => {
        let $dep = $dep_tuple;
    };
    ({ $dep_tuple:expr } $dep:ident $($dep_list:ident)+) => {
        let ($dep, $($dep_list),+) = Clone::clone(&*$dep_tuple);
    };
}

#[macro_export]
macro_rules! __impl_let_dep_list_memo {
    ({ $dep_tuple:expr } $dep:ident) => {
        let $dep = $dep_tuple;
    };
    ({ $dep_tuple:expr } $dep:ident $($dep_list:ident)+) => {
        let ($dep, $($dep_list),+) = $dep_tuple.as_ref();
    };
}

use std::rc::Rc;

pub fn use_memo_no_dep<T: 'static + ?Sized>(func: fn() -> Rc<T>) -> Rc<T> {
    let v = crate::use_ref_readonly_with(move || func());
    v.0
}

pub fn use_memo_one<D: 'static + PartialEq, T: 'static>(
    func: fn(&Rc<D>) -> Rc<T>,
    dep: Rc<D>,
) -> Rc<T> {
    let dep_and_value = crate::use_ref_cell::<Option<(Rc<D>, Rc<T>)>>(None);
    let mut dep_and_value = dep_and_value.0.borrow_mut();

    match &*dep_and_value {
        Some(t) if &t.0 == &dep => {
            // dep not changed
            Rc::clone(&t.1)
        }
        _ => {
            // dep changed
            let new_v: Rc<T> = func(&dep);

            *dep_and_value = Some((dep, Rc::clone(&new_v)));

            new_v
        }
    }
}

#[macro_export]
macro_rules! use_memo {
    (() => $e:expr) => {
        $crate::use_memo_no_dep(|| $crate::auto_wrap_rc!($e))
    };
    (($( $dep:ident $(= $dep_expr:expr)? ),+ $(,)?) => $e:expr ) => {
        $crate::use_memo_one(
            |__dep_tuple| {
                $crate::__impl_let_dep_list_memo!( { *__dep_tuple } $($dep)+ );
                $e
            },
            $crate::auto_wrap_rc!((
                $($crate::__impl_pass_dep!( $dep $(= $dep_expr)? )),+
            )),
        )
    };
}

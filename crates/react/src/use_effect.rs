use std::rc::Rc;

use wasm_bindgen::prelude::*;

pub trait IntoOptionalCleanFn: Sized {
    type CleanFn: 'static + FnOnce();
    fn into_optional_clean_fn(self) -> Option<Self::CleanFn>;
}

impl<F> IntoOptionalCleanFn for F
where
    F: 'static + FnOnce(),
{
    type CleanFn = F;

    #[inline]
    fn into_optional_clean_fn(self) -> Option<F> {
        Some(self)
    }
}

impl<T: IntoOptionalCleanFn> IntoOptionalCleanFn for Option<T> {
    type CleanFn = <T as IntoOptionalCleanFn>::CleanFn;

    #[inline]
    fn into_optional_clean_fn(self) -> Option<Self::CleanFn> {
        self.and_then(IntoOptionalCleanFn::into_optional_clean_fn)
    }
}

impl IntoOptionalCleanFn for () {
    type CleanFn = &'static dyn Fn();

    #[inline]
    fn into_optional_clean_fn(self) -> Option<Self::CleanFn> {
        None
    }
}

fn effect_into_js<C: IntoOptionalCleanFn, F: 'static + FnOnce() -> C>(effect: F) -> JsValue {
    Closure::once_into_js(move || {
        let clean = effect().into_optional_clean_fn();
        if let Some(clean) = clean {
            Closure::once_into_js(move || clean())
        } else {
            JsValue::UNDEFINED
        }
    })
}

/// ```js
/// React.useEffect(() => {
///     effect()
/// })
/// ```
pub fn use_effect_on_each_render<C: IntoOptionalCleanFn, F: 'static + FnOnce() -> C>(effect: F) {
    let effect = effect_into_js(effect);
    react_sys::use_effect_on_each_render(effect);
}

/// ```js
/// React.useEffect(() => {
///     effect()
/// }, [])
/// ```
pub fn use_effect_on_mounted<C: 'static + IntoOptionalCleanFn, F: 'static + FnOnce() -> C>(
    effect: F,
) {
    let ref_effect = react_sys::use_ref(&JsValue::UNDEFINED);
    let mut effect_js = ref_effect.current();
    if effect_js.is_falsy() {
        effect_js = effect_into_js(effect);
        ref_effect.set_current(effect_js.clone());
    }
    react_sys::use_effect(effect_js, js_sys::Array::new());
}

/// `React.useEffect` with exactly one dependency.
/// To use multiple dependencies, see [`use_effect`].
///
/// ```js
/// React.useEffect(() => {
///     effect(dep)
/// }, [dep])
/// ```
///
/// `effect` should be a fn pointer, which can't capture local variables.
/// To capture local variables, call
/// [`use_ref`](super::use_ref::use_ref::use_ref) or
/// [`use_ref_readonly`](crate::use_ref_readonly)
/// to use a ref of that value, then pass the ref as a dependency.
/// (The ref won't change in the component life cycle.)
///
/// For example:
///
/// ```no_run
/// # use wasm_bindgen::JsValue;
/// # use std::rc::Rc;
/// # fn use_test() {
/// let ref_message = react::use_ref_readonly_with(|| Rc::new("hello".to_string()));
/// react::use_effect_one(|ref_message| {
///     let ref_message: &react::ReadRefRc<String> = ref_message.as_ref();
///     let message: &Rc<String> = &ref_message.0;
///     let message: &String = message.as_ref();
///     web_sys::console::log_1(&JsValue::from(format!("{} on component mounted", message)))
/// }, Rc::new(ref_message))
/// # }
/// ```
pub fn use_effect_one<D: 'static + PartialEq, C: 'static + IntoOptionalCleanFn>(
    effect: fn(Rc<D>) -> C,
    dep: Rc<D>,
) {
    let effect_and_dep_arr = super::use_memo_one(
        |v| {
            let (dep, effect) = &**v;
            let dep = Rc::clone(dep);
            let effect = *effect;

            let effect = effect_into_js(move || effect(dep));
            let dep_arr = js_sys::Array::of1(&effect);
            Rc::new((effect, dep_arr))
        },
        Rc::new((dep, effect)),
    );

    let effect = effect_and_dep_arr.0.clone();
    let dep_arr = effect_and_dep_arr.1.clone();
    react_sys::use_effect(effect, dep_arr);
}

///
/// ## `use_effect` with dependencies.
///
/// The closure should not capture any local variables.
/// If you want to use a variable without depend on it,
/// you can `use_ref(value)` then depend on it;
///
///
/// ```no_run
/// # use wasm_bindgen::JsValue;
/// # use react::use_effect;
/// let state = 0;
/// let message = "The state is ";
/// use_effect!((
///     // depend on `state`
///     state,
///     // depend on an expression and name it `message`
///     message = message.to_string(),
/// ) => {
///     web_sys::console::log_2(&JsValue::from(message.as_ref()), &JsValue::from(*state));
/// })
/// ```
///
/// [`use_effect_on_mounted`]
///
/// ```no_run
/// # use wasm_bindgen::JsValue;
/// # use react::use_effect;
/// # let do_something = || {};
/// use_effect!(() => {
///     do_something();
/// })
/// ```
///
/// [`use_effect_on_each_render`]
///
/// ```no_run
/// # use wasm_bindgen::JsValue;
/// # use react::use_effect;
/// # let do_something = || {};
/// use_effect!(on_each_render () => {
///     do_something();
/// })
/// ```
///
#[macro_export]
macro_rules! use_effect {
    (on_each_render () => $e:expr) => {
        $crate::use_effect_on_each_render(move || $e)
    };
    (() => $e:expr) => {
        $crate::use_effect_on_mounted(move || $e)
    };
    (($( $dep:ident $(= $dep_expr:expr)? ),+ $(,)?) => $e:expr ) => {{
        $crate::use_effect_one(
            |dep_tuple| {
                $crate::__impl_let_dep_list!( { dep_tuple } $($dep)+ );
                $e
            },
            {
                let dep = (
                    $($crate::__impl_pass_dep!( $dep $(= $dep_expr)? )),+
                );
                $crate::auto_wrap_rc!(dep)
            },
        )
    }};
}

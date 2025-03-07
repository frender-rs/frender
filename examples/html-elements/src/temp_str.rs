use std::pin::Pin;

use frender::{prelude::*, TempIntoStatic, TempRef, Uncached};

fn use_effect_every_second(
    effect: impl FnMut() + 'static,
) -> hooks::use_effect<(), impl hooks::effect::EffectFor<()>> {
    hooks::use_effect(
        move |(): &()| {
            let timer = gloo::timers::callback::Interval::new(1_000, effect);

            move || drop(timer)
        },
        (),
    )
}

hooks::hook_fn!(
    fn use_time_string() -> &'hook str {
        let (state, set_state) = h![hooks::use_shared_set_with(
            || "<current date time>".to_string()
        )];

        h![use_effect_every_second({
            let set_state = set_state.clone();
            move || set_state.set(js_sys::Date::new_0().to_string().into())
        })];

        state
    }
);

/// equivalent to [`temp_str_with_hook_closure`]
#[cfg(todo)]
fn temp_str_with_element_macro() -> impl Element {
    element!(move || -> TempStr<&'hook str> {
        let s = h![use_time_string()];
        TempRef(s)
    })
}

#[cfg(todo)]
fn temp_str_with_hook_closure() -> impl Element {
    frender::elements::hook_element::new_fn_hook_element(hook_closure!(
        move || -> TempStr<&'hook str> {
            let s = h![use_time_string()];
            TempRef(s)
        }
    ))
}

#[allow(unused)]
fn temp_str_macro_expanded_csr_only() -> impl ::frender::CsrElement {
    #[inline(always)]
    fn identity_fn<
        HookData,
        F: for<'hook> FnMut(Pin<&'hook mut HookData>) -> Uncached<TempRef<'hook, str>>,
    >(
        v: F,
    ) -> F {
        v
    }

    ::frender::elements::hook_element::new_fn_hook_element(identity_fn(
        move |__hooks_hook_0: ::core::pin::Pin<&mut _>| {
            let s = ::frender::__private::hooks_core::UpdateHookUninitialized::h(
                use_time_string(),
                __hooks_hook_0,
            );
            Uncached(TempRef(s))
        },
    ))
}

fn temp_str_macro_expanded() -> impl ::frender::Element {
    #[inline(always)]
    fn identity_fn<
        HookData,
        F: for<'hook> FnMut(Pin<&'hook mut HookData>) -> Uncached<TempIntoStatic<&'hook str>>,
    >(
        v: F,
    ) -> F {
        v
    }

    ::frender::elements::hook_element::new_fn_hook_element(identity_fn(
        move |__hooks_hook_0: ::core::pin::Pin<&mut _>| {
            let s = ::frender::__private::hooks_core::UpdateHookUninitialized::h(
                use_time_string(),
                __hooks_hook_0,
            );
            Uncached(TempIntoStatic(s))
        },
    ))
}

fn temp_str_with_inline_component_fn() -> impl Element {
    component_fn!(move || -> Uncached<TempIntoStatic<&'hook str>> {
        let s = h![use_time_string()];

        Uncached(TempIntoStatic(s))
    })
}

fn temp_str_with_inline_component_fn_with_generics<E: Element + Copy>(el: E) -> impl Element {
    component_fn!(
        for<E: Element + Copy> move || -> (E, Uncached<TempIntoStatic<&'hook str>>) {
            let s = h![use_time_string()];

            (el, Uncached(TempIntoStatic(s)))
        }
    )
}

pub fn main() -> impl Element {
    (
        temp_str_macro_expanded(),
        temp_str_with_inline_component_fn(),
        temp_str_with_inline_component_fn_with_generics("This is an example with generics:"),
    )
}

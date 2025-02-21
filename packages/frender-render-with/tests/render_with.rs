#![cfg(feature = "csr")]
#![allow(dead_code)] // TODO: add tests

use std::cell::RefCell;

use frender_common::TempStr;
use frender_html::cs;
use frender_keyed_elements::{Keyed, KeyedElements};
use hooks::ShareValue;

use frender_render_with::{FnOnceRenderWithContext, IntoFnOnceRenderWithContext, RenderHtml};

struct Test {
    numbers: Vec<i32>,
}

impl IntoFnOnceRenderWithContext for Test {
    fn into_fn_once_render_with_context<Renderer: ?Sized + RenderHtml>(
        self,
    ) -> impl FnOnceRenderWithContext<Renderer> {
        move |ctx| {
            ctx.render((
                cs::button()
                    .children(TempStr(&*String::new()))
                    .on_click(|_: &_| {}),
                KeyedElements(self.numbers.iter().map(|i| Keyed(*i, *i))),
            ))
        }
    }
}

struct TestShareValue<S: ShareValue<Value = String>>(S);

impl<S: ShareValue<Value = String>> IntoFnOnceRenderWithContext for TestShareValue<S> {
    fn into_fn_once_render_with_context<Renderer: ?Sized + RenderHtml>(
        self,
    ) -> impl FnOnceRenderWithContext<Renderer> {
        move |ctx| self.0.map(|s| ctx.render(TempStr(s.as_str())))
    }
}

struct TestRcRefCellElements(std::rc::Rc<RefCell<Vec<i32>>>);
impl IntoFnOnceRenderWithContext for TestRcRefCellElements {
    fn into_fn_once_render_with_context<Renderer: ?Sized + RenderHtml>(
        self,
    ) -> impl FnOnceRenderWithContext<Renderer> {
        move |ctx| {
            let numbers = self.0.borrow();

            ctx.render(KeyedElements(numbers.iter().map(|n| Keyed(*n, *n))))
        }
    }
}

struct TestShareElements<S: ShareValue<Value = Vec<i32>>>(S);

impl<S: ShareValue<Value = Vec<i32>>> IntoFnOnceRenderWithContext for TestShareElements<S> {
    fn into_fn_once_render_with_context<Renderer: ?Sized + RenderHtml>(
        self,
    ) -> impl FnOnceRenderWithContext<Renderer> {
        move |ctx| {
            self.0
                .map(|numbers| ctx.render(KeyedElements(numbers.iter().map(|n| Keyed(*n, *n)))))
        }
    }
}

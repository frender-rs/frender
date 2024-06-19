use std::cell::RefCell;

use frender::prelude::*;
use frender_common::TempStr;
use frender_html::RenderHtml;
use hooks::ShareValue;

use frender_render_with::{
    CsrRenderContext, DefaultAnyRenderState, FnOnceRenderWithContext, Rendered,
};

struct Test {
    numbers: Vec<i32>,
}

impl FnOnceRenderWithContext for Test {
    fn call_once_render_with_context<'r, PEH: ?Sized, Renderer: ?Sized + RenderHtml>(
        self,
        ctx: CsrRenderContext<'r, PEH, Renderer>,
    ) -> Rendered<'r, impl DefaultAnyRenderState<PEH, Renderer>> {
        ctx.render((
            cs::button
                .children(TempStr(&*String::new()))
                .on_click(|_: &_| {}),
            Elements(self.numbers.iter().map(|i| Keyed(*i, *i))),
        ))
    }
}

struct TestShareValue<S: ShareValue<Value = String>>(S);

impl<S: ShareValue<Value = String>> FnOnceRenderWithContext for TestShareValue<S> {
    fn call_once_render_with_context<'r, PEH: ?Sized, Renderer: ?Sized + RenderHtml>(
        self,
        ctx: CsrRenderContext<'r, PEH, Renderer>,
    ) -> Rendered<'r, impl DefaultAnyRenderState<PEH, Renderer>> {
        self.0.map(|s| ctx.render(TempStr(s.as_str())))
    }
}

struct TestRcRefCellElements(std::rc::Rc<RefCell<Vec<i32>>>);
impl FnOnceRenderWithContext for TestRcRefCellElements {
    fn call_once_render_with_context<'r, PEH: ?Sized, Renderer: ?Sized + RenderHtml>(
        self,
        ctx: CsrRenderContext<'r, PEH, Renderer>,
    ) -> Rendered<'r, impl DefaultAnyRenderState<PEH, Renderer>> {
        let numbers = self.0.borrow();

        ctx.render(Elements(numbers.iter().map(|n| Keyed(*n, *n))))
    }
}

struct TestShareElements<S: ShareValue<Value = Vec<i32>>>(S);

impl<S: ShareValue<Value = Vec<i32>>> FnOnceRenderWithContext for TestShareElements<S> {
    fn call_once_render_with_context<'r, PEH: ?Sized, Renderer: ?Sized + RenderHtml>(
        self,
        ctx: CsrRenderContext<'r, PEH, Renderer>,
    ) -> Rendered<'r, impl DefaultAnyRenderState<PEH, Renderer>> {
        self.0
            .map(|numbers| ctx.render(Elements(numbers.iter().map(|n| Keyed(*n, *n)))))
    }
}

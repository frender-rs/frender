pub trait HookContext: Sized {
    fn take_context(this: &mut Self) -> Self;
    fn with_context(this: &mut Self, f: impl FnOnce(&mut Self));
}

#[cfg(feature = "dom")]
impl HookContext for frender_dom::Dom {
    #[inline]
    fn take_context(this: &mut Self) -> Self {
        this.clone()
    }

    #[inline]
    fn with_context(this: &mut Self, f: impl FnOnce(&mut Self)) {
        this.with_position(f)
    }
}

#[cfg(feature = "ssr")]
impl<W: frender_ssr::AsyncWrite + Unpin> HookContext for frender_ssr::SsrContext<W> {
    #[inline]
    fn take_context(this: &mut Self) -> Self {
        std::mem::take(this)
    }

    #[inline]
    fn with_context(this: &mut Self, f: impl FnOnce(&mut Self)) {
        f(this)
    }
}

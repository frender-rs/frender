use crate::{Element, RenderState};

impl RenderState for () {
    fn poll_render<W: crate::AsyncWrite + ?Sized>(
        self: std::pin::Pin<&mut Self>,
        _: std::pin::Pin<&mut W>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::task::Poll::Ready(Ok(()))
    }
}

impl Element for () {
    type SsrState = ();

    fn into_ssr_state(self) -> Self::SsrState {}

    type HtmlChildren = crate::Empty;

    fn into_html_children(self) -> Self::HtmlChildren {
        crate::Empty
    }
}

impl<R0: Element> Element for (R0,) {
    type SsrState = R0::SsrState;

    fn into_ssr_state(self) -> Self::SsrState {
        R0::into_ssr_state(self.0)
    }

    type HtmlChildren = R0::HtmlChildren;

    fn into_html_children(self) -> Self::HtmlChildren {
        self.0.into_html_children()
    }
}

macro_rules! impl_render_for_tuple {
    ($(($($field:ident),+) ,)+) => {
        $(
            impl<$($field: RenderState),+> RenderState for ($($field,)+) {
                fn poll_render<W: crate::AsyncWrite + ?Sized>(
                    self: std::pin::Pin<&mut Self>,
                    mut writer: std::pin::Pin<&mut W>,
                    cx: &mut std::task::Context<'_>,
                ) -> std::task::Poll<std::io::Result<()>> {
                    #[allow(non_snake_case)]
                    // SAFETY: pin projection
                    let ($($field,)+) = unsafe {
                        match std::pin::Pin::get_unchecked_mut(self) {
                            ($($field,)+) => (
                                $(std::pin::Pin::new_unchecked($field),)+
                            )
                        }
                    };

                    $(
                        crate::ready_ok!($field::poll_render($field, writer.as_mut(), cx));
                    )+

                    std::task::Poll::Ready(Ok(()))
                }
            }

            impl<$($field: Element),+> Element for ($($field),+) {
                type SsrState = ($($field::SsrState,)+);

                #[inline]
                fn into_ssr_state(self) -> Self::SsrState {
                    #[allow(non_snake_case)]
                    let ($($field,)+) = self;

                    ($(
                        $field::into_ssr_state($field),
                    )+)
                }

                type HtmlChildren = <
                    async_str_iter::concat::Concat<($($field::HtmlChildren),+)>
                    as async_str_iter::IntoAsyncStrIterator
                >::IntoAsyncStrIterator;

                fn into_html_children(self) -> Self::HtmlChildren {
                    let ($($field,)+) = self;
                    async_str_iter::IntoAsyncStrIterator::into_async_str_iterator(
                        async_str_iter::concat::Concat(
                            ($($field.into_html_children(),)+)
                        )
                    )
                }
            }
        )+
    };
}

impl_render_for_tuple! {
    (R0, R1),
    (R0, R1, R2),
    (R0, R1, R2, R3),
    (R0, R1, R2, R3, R4),
    (R0, R1, R2, R3, R4, R5),
    (R0, R1, R2, R3, R4, R5, R6),
    (R0, R1, R2, R3, R4, R5, R6, R7),
    (R0, R1, R2, R3, R4, R5, R6, R7, R8),
    (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9),
    (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10),
    (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11),
    (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12),
}

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

    type IntoIterHtmlChunk = crate::str_iter::Empty;

    fn into_iter_html_chunk(self) -> Self::IntoIterHtmlChunk {
        crate::str_iter::Empty
    }
}

impl<R0: Element> Element for (R0,) {
    type SsrState = R0::SsrState;

    fn into_ssr_state(self) -> Self::SsrState {
        R0::into_ssr_state(self.0)
    }

    type IntoIterHtmlChunk = R0::IntoIterHtmlChunk;

    fn into_iter_html_chunk(self) -> Self::IntoIterHtmlChunk {
        self.0.into_iter_html_chunk()
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

            frender_common::expand! {
                {$({$field})+} get {-1}
                prepend(
                    #[allow(non_snake_case)]
                    pub mod
                )
                append({
                    use crate::Element;

                    crate::Strings! {
                        enum State {}
                        pub struct Strings<$($field: crate::AsyncStrIterator),+>(
                            $($field!($field),)+
                        );
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

                        type IntoIterHtmlChunk = self::Strings<$($field::IntoIterHtmlChunk),+>;

                        fn into_iter_html_chunk(self) -> Self::IntoIterHtmlChunk {
                            let ($($field,)+) = self;
                            self::Strings {
                                _state: self::State(),
                                $($field: $field.into_iter_html_chunk(),)+
                            }
                        }
                    }
                })
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

use crate::SsrElement;

impl SsrElement for () {
    type HtmlChildren = crate::Empty;

    fn into_html_children(self) -> Self::HtmlChildren {
        crate::Empty
    }
}

impl<R0: SsrElement> SsrElement for (R0,) {
    type HtmlChildren = R0::HtmlChildren;

    fn into_html_children(self) -> Self::HtmlChildren {
        self.0.into_html_children()
    }
}

macro_rules! impl_render_for_tuple {
    ($(($($field:ident),+) ,)+) => {
        $(
            impl<$($field: SsrElement),+> SsrElement for ($($field),+) {
                type HtmlChildren = <
                    async_str_iter::concat::Concat<($($field::HtmlChildren),+)>
                    as async_str_iter::IntoAsyncStrIterator
                >::IntoAsyncStrIterator;

                fn into_html_children(self) -> Self::HtmlChildren {
                    #[allow(non_snake_case)]
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

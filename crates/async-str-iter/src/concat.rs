#![allow(non_snake_case)]

use crate::IntoAsyncStrIterator;

pub struct Concat<T>(pub T);

impl IntoAsyncStrIterator for Concat<()> {
    type IntoAsyncStrIterator = super::empty::Empty;

    fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
        super::empty::Empty
    }
}

macro_rules! impl_for_tuple {
    ($($iter:ident = $state:ident ($($field:ident),+) ,)+) => {
        $(
            crate::Strings! {
                enum $state {}
                pub struct $iter<$($field: crate::AsyncStrIterator),+>(
                    $(
                        $field!($field),
                    )+
                );
            }

            impl<$($field: IntoAsyncStrIterator),+> IntoAsyncStrIterator for Concat<($($field),+)> {
                type IntoAsyncStrIterator = self::$iter<$($field::IntoAsyncStrIterator,)+>;

                fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
                    #[allow(non_snake_case)]
                    let Self(($($field,)+)) = self;
                    self::$iter {
                        _state: self::$state(),
                        $($field: $field.into_async_str_iterator(),)+
                    }
                }
            }
        )+
    };
}

impl_for_tuple!(
    IterTuple2 = IterTuple2State(R0, R1),
    IterTuple3 = IterTuple3State(R0, R1, R2),
    IterTuple4 = IterTuple4State(R0, R1, R2, R3),
    IterTuple5 = IterTuple5State(R0, R1, R2, R3, R4),
    IterTuple6 = IterTuple6State(R0, R1, R2, R3, R4, R5),
    IterTuple7 = IterTuple7State(R0, R1, R2, R3, R4, R5, R6),
    IterTuple8 = IterTuple8State(R0, R1, R2, R3, R4, R5, R6, R7),
    IterTuple9 = IterTuple9State(R0, R1, R2, R3, R4, R5, R6, R7, R8),
    IterTuple10 = IterTuple10State(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9),
    IterTuple11 = IterTuple11State(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10),
    IterTuple12 = IterTuple12State(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11),
    IterTuple13 = IterTuple13State(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12),
);

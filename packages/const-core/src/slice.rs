mod iter;
pub use iter::Iter;

/// [`<[_] as PartialEq>::eq()`](https://doc.rust-lang.org/stable/std/primitive.slice.html#impl-PartialEq%3C%5BU%5D%3E-for-%5BT%5D)
/// implemented by comparing [`len`](slice::len) and then item by item.
#[macro_export]
macro_rules! slice_eq {
    ($slice_a:expr, $slice_b:expr $(,)?) => {
        $crate::slice_eq!($slice_a, $slice_b, ne = |a, b| *a != *b)
    };
    ($slice_a:expr, $slice_b:expr, eq = |$item_a:pat_param, $item_b:pat_param $(,)?| $eq:expr $(,)?) => {
        $crate::slice_eq!($slice_a, $slice_b, ne = |$item_a, $item_b| !$eq)
    };
    ($slice_a:expr, $slice_b:expr, ne = |$item_a:pat_param, $item_b:pat_param $(,)?| $ne:expr $(,)?) => {{
        let slice_a: &[_] = $slice_a;
        let slice_b: &[_] = $slice_b;

        slice_a.len() == slice_b.len() && {
            let mut i = 0;
            let mut res = true;
            while i < slice_a.len() {
                let $item_a = &slice_a[i];
                let $item_b = &slice_b[i];
                if $ne {
                    res = false;
                    break;
                }
                i += 1;
            }

            res
        }
    }};
}

/// [`<[_]>::sort()`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.sort)
#[macro_export]
macro_rules! slice_sort {
    ($unsorted:expr $(,)?) => {
        $crate::slice_sort!($unsorted, gt = |a, b| *a > *b)
    };
    ($unsorted:expr, gt = |$a:pat_param, $b:pat_param $(,)?| $gt:expr $(,)?) => {
        match <[_]>::split_first_mut($unsorted) {
            $crate::slice::__private::__sort::Some((first, [second, rest @ ..])) => {
                let mut rest = rest;
                loop {
                    {
                        let mut first = &mut *first;
                        let mut second = &mut *second;
                        let mut rest = &mut *rest;

                        loop {
                            let $a = &*first;
                            let $b = &*second;
                            if $gt {
                                $crate::slice::__private::__sort::swap(first, second);
                            }
                            if let $crate::slice::__private::__sort::Some((new_second, new_rest)) =
                                <[_]>::split_first_mut(rest)
                            {
                                first = second;
                                second = new_second;
                                rest = new_rest;
                            } else {
                                break;
                            }
                        }
                    }
                    // now rest.last() is the largest
                    match <[_]>::split_last_mut(rest) {
                        $crate::slice::__private::__sort::Some((_, new_rest)) => {
                            rest = new_rest;
                        }
                        $crate::slice::__private::__sort::None => {
                            break;
                        }
                    }
                }
            }
            $crate::slice::__private::__sort::Some((_, []))
            | $crate::slice::__private::__sort::None => {}
        }
    };
}

pub mod __private {
    pub mod __sort {
        pub use {::core::mem::swap, None, Some};
    }
}

#[cfg(test)]
mod tests {
    #[test]
    const fn slice_eq() {
        const EMPTY_U8: [u8; 0] = [];
        assert!(slice_eq!(EMPTY_U8.as_slice(), &[], ne = |a, b| *a != *b,));

        struct NonCopy;
        assert!(slice_eq!(
            &[NonCopy],
            [NonCopy].as_slice(),
            eq = |_, _| true,
        ));

        assert!(slice_eq!([1].as_slice(), [1].as_slice()));

        assert!(!slice_eq!(&[1], &[0]));
        assert!(!slice_eq!(&[1], &[]));
        assert!(!slice_eq!(&[1], &[1, 2]));
    }

    const _: () = slice_eq();

    fn test_u8_slice_sort<const N: usize>(data: &[u8; N]) {
        let mut data_1 = *data;
        let mut data_2 = *data;
        slice_sort!(&mut data_1);
        data_2.sort();

        assert_eq!(data_1, data_2);
    }

    #[test]
    fn slice_sort() {
        test_u8_slice_sort(b"");
        test_u8_slice_sort(b"0");
        test_u8_slice_sort(b"128r7g");
        test_u8_slice_sort(b"98asdbgui2qu");
        test_u8_slice_sort(b"87qg2buafs");
    }
}

pub trait AsAttr<R> {
    fn as_attr(self) -> R;
}

impl<R> AsAttr<Option<R>> for R {
    fn as_attr(self) -> Option<R> {
        Some(self)
    }
}

impl<R> AsAttr<R> for R {
    fn as_attr(self) -> R {
        self
    }
}

// impl<'a, N: crate::Node> AsAttr<&'a dyn crate::Node> for &'a N {
//     fn as_attr(self) -> &'a dyn crate::Node {
//         self
//     }
// }

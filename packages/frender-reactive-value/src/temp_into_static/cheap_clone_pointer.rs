use std::{borrow::Borrow, ops::Deref};

/// CheapClone
pub(crate) trait KnownCheapClonePointer:
    Deref<Target: 'static + PartialEq> + 'static + Borrow<Self::Target> + Clone + PartialEq
{
}

impl<T: ?Sized + 'static + PartialEq> KnownCheapClonePointer for std::rc::Rc<T> {}
impl<T: ?Sized + 'static + PartialEq> KnownCheapClonePointer for std::sync::Arc<T> {}

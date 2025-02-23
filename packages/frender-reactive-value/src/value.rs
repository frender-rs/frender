use crate::value_kind::{KindOfOwned, ValueKind};

trait ValueKindOfValue: ValueKind {
    type This;
}

pub trait AsValueOfKind<VK: ValueKind> {}

pub trait Value<VK: ValueKind> {}

impl AsValueOfKind<KindOfOwned<String>> for String {}

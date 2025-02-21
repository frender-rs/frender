use super::ReactiveValueWithKind;

pub struct ReactiveValueIntoElement<V: ReactiveValueWithKind>(pub V);

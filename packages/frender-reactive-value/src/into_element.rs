use super::ReactiveValueWithKind;

#[derive(Debug, Clone, Copy)]
pub struct ReactiveValueIntoElement<V: ReactiveValueWithKind>(pub V);

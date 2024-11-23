#[derive(Debug, Default)]
/// The attribute type marker prevents different attributes with same type of state to be replaced by each other.
pub struct AttributeState<AttrMarker, S>(pub AttrMarker, pub S);

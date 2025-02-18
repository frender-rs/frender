pub trait IntoProperty {
    type IntoProperty;
    fn into_property(this: Self) -> Self::IntoProperty;
}

pub trait IntoPropValue<R> {
    fn into_prop_value(self) -> R;
}

impl<R> IntoPropValue<Option<R>> for R {
    fn into_prop_value(self) -> Option<R> {
        Some(self)
    }
}

impl<R> IntoPropValue<R> for R {
    fn into_prop_value(self) -> R {
        self
    }
}

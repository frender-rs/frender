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

#[cfg(test)]
mod tests {
    use super::IntoPropValue;

    #[test]
    fn simple() {
        let _: i32 = IntoPropValue::into_prop_value(0i32);
        let _: Option<i32> = IntoPropValue::into_prop_value(0i32);
        let _: Option<i32> = IntoPropValue::into_prop_value(Some(0i32));
        let _: Option<i32> = IntoPropValue::into_prop_value(None);
    }

    #[test]
    fn impl_for_custom_type() {
        struct MyNum(f64);

        impl IntoPropValue<MyNum> for f64 {
            fn into_prop_value(self) -> MyNum {
                MyNum(self)
            }
        }

        let _: MyNum = IntoPropValue::into_prop_value(0.0);
    }

    #[test]
    fn with_trait() {
        struct MyNum(i32);

        impl IntoPropValue<i32> for MyNum {
            fn into_prop_value(self) -> i32 {
                self.0
            }
        }

        fn display<D: std::fmt::Display>(v: D) -> String {
            v.to_string()
        }

        assert_eq!(display::<i32>(IntoPropValue::into_prop_value(1)), "1");
        assert_eq!(display::<&str>(IntoPropValue::into_prop_value("2")), "2");
        assert_eq!(
            display::<i32>(IntoPropValue::into_prop_value(MyNum(3))),
            "3"
        );
    }

    #[test]
    fn with_option_trait() {
        fn display<D: std::fmt::Display>(v: Option<D>) -> String {
            v.as_ref()
                .map_or_else(|| String::new(), ToString::to_string)
        }

        assert_eq!(display(IntoPropValue::into_prop_value(1)), "1");
        assert_eq!(display(IntoPropValue::into_prop_value("2")), "2");
        assert_eq!(display::<f64>(IntoPropValue::into_prop_value(None)), "");
    }
}

use chtml::encode::attribute_value::AttributeValueForRendering;

use crate::{
    attr_value,
    values::r#const::{ConstAttrValue, HasConstAttrValue},
};

const fn assert_empty<M: HasConstAttrValue<AttrValue = AttributeValueForRendering<'static, 0>>>(
    _: ConstAttrValue<M>,
) {
    const {
        assert!(M::ATTR_VALUE.as_str().is_empty());
        assert!(matches!(
            M::ATTR_VALUE.as_eq_value_str().as_str().as_bytes(),
            b"=\"\""
        ));
    }
}

#[test]
fn compile_only() {
    const {
        assert_empty(attr_value::r#const!(const { "" }));
        assert_empty(attr_value::r#const!(const { "" } as _));
        assert_empty(attr_value::r#const!(const { "" } as &'static str));
        assert_empty(attr_value::r#const!(""));
        assert_empty(attr_value::r#const!("" as _));
        assert_empty(attr_value::r#const!("" as &'static str));
        const EMPTY: &str = "";
        assert_empty(attr_value::r#const!(EMPTY));
    }
}

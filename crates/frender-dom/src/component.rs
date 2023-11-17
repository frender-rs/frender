pub trait HasIntrinsicComponentTag {
    const INTRINSIC_COMPONENT_TAG: &'static str;
}

pub trait HasIntrinsicElementType {
    type IntrinsicElementType: IntrinsicElementType;
}

pub trait IntrinsicElementType {}

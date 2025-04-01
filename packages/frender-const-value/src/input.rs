pub use const_core::convert::{const_into, ConstFrom, ConstInto, MarkerOfConstValue};

pub trait Input<For>: ConstInto<Self::Temp> {
    type Temp: ConstInto<Info>;
}

const INFO_LEN: usize = 4;
pub trait InputWithInfo<
    For,
    const INFO_0: usize = DUMMY_INFO,
    const INFO_1: usize = DUMMY_INFO,
    const INFO_2: usize = DUMMY_INFO,
    const INFO_3: usize = DUMMY_INFO,
>: Input<For, Temp: ConstInto<Self::Output>>
{
    type Output;
}

#[macro_export]
macro_rules! as_InputWithInfo {
    (<$Ty:ty as $InputWithInfo:ident $(::)? <$For:ty, $info:block>> $($rest:tt)*) => {
        <$Ty as $crate::input::InputWithInfo::<
            $For,
            { $info.0[0] },
            { $info.0[1] },
            { $info.0[2] },
            { $info.0[3] },
        >> $($rest)*
    };
}

const DUMMY_INFO: usize = 0xaa_usize;
pub struct Info(pub [usize; INFO_LEN]);

impl Info {
    pub const fn new() -> Self {
        Self([DUMMY_INFO; INFO_LEN])
    }

    pub const fn new_1(info_0: usize) -> Self {
        let mut this = Self::new();
        this.0[0] = info_0;
        this
    }
}

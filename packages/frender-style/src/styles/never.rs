use crate::Style;

pub enum Never {}

impl Style for Never {}

impl Never {
    pub fn assert(this: Self) -> Self {
        this
    }
}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

#[cfg(test)]
#[test]
fn as_never() {
    fn cast(never: std::convert::Infallible) -> Never {
        (|| -> Never { match never {} })()
    }

    None::<std::convert::Infallible>.map(cast);
}

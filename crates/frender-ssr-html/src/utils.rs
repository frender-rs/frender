/// If len is 0, `true` is returned.
macro_rules! bytes_all {
    ($bytes:expr, |$b:pat_param $(,)?| $condition:expr $(,)?) => {
        'bytes_all: {
            let bytes: &[::core::primitive::u8] = &$bytes;
            let mut i = 0usize;

            while i < bytes.len() {
                let $b = bytes[i];
                if $condition {
                    i += 1;
                } else {
                    break 'bytes_all false;
                }
            }

            true
        }
    };
}

pub(crate) use bytes_all;

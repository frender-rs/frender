pub use self::array_string::ArrayString;

mod array_string;

pub const fn put_at<'a, T, const N: usize>(
    mut arr: [T; N],
    mut at: usize,
    items: &[T],
) -> ([T; N], usize)
where
    T: Copy,
{
    let mut j = 0;
    while j < items.len() {
        arr[at] = items[j];
        at += 1;
        j += 1;
    }

    (arr, at)
}

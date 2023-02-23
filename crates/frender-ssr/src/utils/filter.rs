pub struct FilterIdentity<I>(pub I);

impl<I, T> Iterator for FilterIdentity<I>
where
    I: Iterator<Item = Option<T>>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(Some(v)) => return Some(v),
                None => return None,
                Some(None) => continue,
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

pub type FilterArray<T, const N: usize> = FilterIdentity<::core::array::IntoIter<Option<T>, N>>;

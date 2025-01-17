#[derive(Debug, Clone, Copy)]
pub struct Keyed<K, E>(pub K, pub E);

impl<K, E> Keyed<K, E> {
    pub fn from_tuple((key, element): (K, E)) -> Self {
        Keyed(key, element)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultAlgorithm;

#[derive(Debug, Clone, Copy)]
pub struct KeyedElements<I: IntoIterator, A = DefaultAlgorithm> {
    pub iter: I,
    pub algorithm: A,
}

#[allow(non_snake_case)]
pub const fn KeyedElements<I: IntoIterator>(iter: I) -> KeyedElements<I> {
    KeyedElements {
        iter,
        algorithm: DefaultAlgorithm,
    }
}

#[cfg(feature = "ssr")]
pub mod ssr;

#[cfg(feature = "csr")]
pub mod csr;

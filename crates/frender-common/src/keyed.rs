#[derive(Debug, Clone, Copy)]
pub struct Keyed<K, E>(pub K, pub E);

impl<K, E> Keyed<K, E> {
    pub fn from_tuple((key, element): (K, E)) -> Self {
        Keyed(key, element)
    }
}

pub struct DefaultElementsAlgorithm;

#[derive(Debug, Clone, Copy)]
pub struct Elements<I: IntoIterator, A = DefaultElementsAlgorithm> {
    pub iter: I,
    pub algorithm: A,
}

#[allow(non_snake_case)]
pub fn Elements<I: IntoIterator>(iter: I) -> Elements<I> {
    Elements {
        iter,
        algorithm: DefaultElementsAlgorithm,
    }
}

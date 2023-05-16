/// TODO: `Keyed(element, key)`
///     - improve performance.
///       Currently, all elements are unmounted and then new elements update the states.
///     - impl UpdateRenderState<Ctx> for T where T: IntoIterator<Keyed<E, K>>
#[derive(Debug, Clone, Copy)]
pub struct Keyed<K, E>(pub K, pub E);

#[derive(Debug, Clone, Copy)]
pub struct Elements<I: IntoIterator>(pub I);

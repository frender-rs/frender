#[derive(Debug, Clone, Copy)]
pub enum EitherElement<A, B> {
    A(A),
    B(B),
}

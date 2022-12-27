#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

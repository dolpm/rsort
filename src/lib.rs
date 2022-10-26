pub mod components;
pub mod sorts;

#[derive(Clone, Eq, PartialEq)]
pub enum Msg<T> {
    Data(Vec<T>),
}

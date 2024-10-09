use std::marker::PhantomData;

use crate::MaybeSend;

trait Seal {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Just<T>(pub(crate) T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nothing<T>(pub(crate) PhantomData<T>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Async {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Blocking {}

impl<T> Default for Nothing<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

pub trait ToOption: Seal + MaybeSend {
    type Some: MaybeSend;

    fn maybe(self) -> Option<Self::Some>;
}

impl<T> Seal for Just<T> {}
impl<T: MaybeSend> ToOption for Just<T> {
    type Some = T;
    fn maybe(self) -> Option<T> {
        Some(self.0)
    }
}

impl<T> Seal for Nothing<T> {}
impl<T: MaybeSend> ToOption for Nothing<T> {
    type Some = T;
    fn maybe(self) -> Option<T> {
        None
    }
}

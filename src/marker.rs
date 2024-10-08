use std::marker::PhantomData;

use crate::MaybeSend;

trait Seal {}

pub struct Just<T>(pub(crate) T);
pub struct Nothing<T>(pub(crate) PhantomData<T>);
pub struct Async {}
pub struct Blocking {}

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

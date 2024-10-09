#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(clippy::nursery, clippy::pedantic, missing_docs)]
#![allow(
    private_interfaces,
    private_bounds,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::redundant_pub_crate,
    clippy::module_name_repetitions
)]
#![cfg_attr(not(feature = "send"), allow(clippy::future_not_send))]

mod_use::mod_use![r#async, blocking, error, builder, shared];
mod format;
mod marker;

use std::{convert::Infallible, marker::PhantomData, path::Path};

use format::Format;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    format::DefaultFormat,
    marker::{Async, Blocking, Nothing},
};

const BUF_SIZE: usize = 1 << 8; // 256 bytes

/// Trait alias for types that can be serialized and deserialized.
pub trait Data: Serialize + DeserializeOwned + MaybeSend {}

impl<T> Data for T where T: Serialize + DeserializeOwned + MaybeSend {}

/// Synchronously binds data to a file.
#[derive(Debug, Clone)]
pub struct Vow<T, F> {
    pub(crate) value: Option<T>,
    pub(crate) io: Io<F, Blocking>,
}

/// Asynchronously binds data to a file.
#[derive(Debug, Clone)]
pub struct VowAsync<T, F> {
    pub(crate) value: Option<T>,
    pub(crate) io: Io<F, Async>,
}

shared_impl!(Vow<T, F: VowFile>);
shared_impl!(VowAsync<T, F: VowFileAsync>, async + await);

impl<T, F> Vow<T, F>
where
    T: Data,
    F: VowFile,
{
    /// Create a new [`Vow`] instance with content stored in `file`.
    ///
    /// This will fail if the file contains invalid data.
    pub fn new(file: F) -> VowResult<Self> {
        VowBuilder::<_, _, Blocking, _>::new(file).build()
    }
}

impl Vow<Infallible, std::fs::File> {
    /// Open a blocking file at the given path.
    pub fn open<P: AsRef<Path>>(
        path: P,
    ) -> VowBuilder<Nothing<Infallible>, std::fs::File, Blocking, DefaultFormat> {
        VowBuilder::<_, _, Blocking, _>::open(path)
    }
}

impl<F> Vow<Infallible, F> {
    /// Create a new builder for [`Vow`].
    pub const fn builder(file: F) -> VowBuilder<Nothing<Infallible>, F, Blocking, DefaultFormat> {
        VowBuilder::<_, _, Blocking, _>::new(file)
    }
}

impl<T, F> VowAsync<T, F>
where
    T: Data,
    F: VowFileAsync,
{
    /// Create a new [`Vow`] instance asynchronously with content stored in `file`.
    ///
    /// This will fail if the file contains invalid data.
    pub async fn new(file: F) -> VowResult<Self> {
        VowBuilder::<_, _, Async, _>::new(file).build().await
    }
}

#[cfg(feature = "backend-tokio")]
impl VowAsync<Infallible, tokio::fs::File> {
    /// Open a `tokio` file at the given path.
    pub fn open_tokio<P: AsRef<Path>>(
        path: P,
    ) -> VowBuilder<Nothing<Infallible>, tokio::fs::File, Async, DefaultFormat> {
        VowBuilder::<_, _, Async, _>::open(path)
    }
}

#[cfg(feature = "backend-async-std")]
impl VowAsync<Infallible, async_std::fs::File> {
    /// Open a `tokio` file at the given path.
    pub fn open_async_std<P: AsRef<Path>>(
        path: P,
    ) -> VowBuilder<Nothing<Infallible>, async_std::fs::File, Async, DefaultFormat> {
        VowBuilder::<_, _, Async, _>::open(path)
    }
}

#[cfg(feature = "backend-compio")]
impl VowAsync<Infallible, compio_fs::File> {
    /// Open a `compio` file at the given path.
    pub fn open_compio<P: AsRef<Path>>(
        path: P,
    ) -> VowBuilder<Nothing<Infallible>, compio_fs::File, Async, DefaultFormat> {
        VowBuilder::<_, _, Async, _>::open(path)
    }
}

impl<T, F: VowFileAsync> VowAsync<T, F> {
    /// Create a new builder for [`VowAsync`].
    pub const fn builder(file: F) -> VowBuilder<Nothing<T>, F, Async, DefaultFormat>
    where
        F: VowFileAsync,
    {
        VowBuilder::<_, _, Async, _>::new(file)
    }
}
/// Underlying file operations.
#[derive(Debug, Clone)]
struct Io<F, A> {
    pub(crate) file: F,
    pub(crate) buf: Vec<u8>,
    pub(crate) format: Format,
    pub(crate) asyncness: PhantomData<A>,
    pub(crate) deny_invalid: bool,
}

impl<F, A> Io<F, A> {
    pub fn new(file: F, format: Format, deny_invalid: bool) -> Self {
        Self {
            file,
            buf: Vec::with_capacity(BUF_SIZE),
            format,
            deny_invalid,
            asyncness: PhantomData,
        }
    }

    pub fn take_buf(&mut self) -> Vec<u8> {
        let mut buf = std::mem::take(&mut self.buf);
        buf.clear();
        buf
    }
}

#[cfg(docsrs)]
compile_error!("docs.rs");

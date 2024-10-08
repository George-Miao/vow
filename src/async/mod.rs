#![allow(clippy::module_name_repetitions)]

use std::{future::Future, io, path::Path};

use crate::{format, Async, Data, Io, VowResult};

mod_use::mod_use![maybe_send];

#[cfg(feature = "backend-tokio")]
mod tokio;

#[cfg(feature = "backend-compio")]
mod compio;

#[cfg(feature = "backend-async-std")]
mod async_std;

macro_rules! tri {
    ($self:ident, $expr:expr) => {{
        let (res, buf) = $expr.await;
        $self.buf = buf;
        res?
    }};
}

use tri;

/// Trait alias for futures returning `io::Result<T>`
pub trait IoFut<T>: Future<Output = io::Result<T>> + MaybeSend {}

impl<F, T> IoFut<T> for F where F: ?Sized + Future<Output = io::Result<T>> + MaybeSend {}

/// Trait alias for futures returning `(io::Result<()>, Vec<u8>)`
pub trait BufFut: Future<Output = (io::Result<()>, Vec<u8>)> + MaybeSend {}

impl<F> BufFut for F where F: ?Sized + Future<Output = (io::Result<()>, Vec<u8>)> + MaybeSend {}

/// Low-level trait for asynchronous file operations
pub trait VowFileAsync: MaybeSend {
    /// Open a new file at the given path asynchronously
    fn open(path: &Path) -> impl IoFut<Self>
    where
        Self: Sized;

    /// Read **entire** file into a buffer
    fn read(&mut self, buf: Vec<u8>) -> impl BufFut;

    /// Write **entire** buffer into a file
    fn write(&mut self, buf: Vec<u8>) -> impl BufFut;

    /// Flush the file
    fn flush(&mut self) -> impl IoFut<()>;

    /// Set the length of the file
    fn set_len(&mut self, len: u64) -> impl IoFut<()>;
}

impl<F: VowFileAsync> Io<F, Async> {
    pub(crate) async fn sync<T: Data>(
        &mut self,
        current: Option<T>,
        overwrite: bool,
    ) -> VowResult<T> {
        let mut buf = self.take_buf();

        if let Some(current) = current {
            let ret = if overwrite {
                format::se(self.format, buf.as_mut(), &current)?;
                self.file.set_len(0).await?;
                tri!(self, self.file.write(buf));
                current
            } else {
                tri!(self, self.file.read(buf));
                match format::des(self.format, &self.buf) {
                    Ok(value) => value,
                    Err(err) => {
                        if err.is_invalid_data() {
                            if self.deny_invalid {
                                return Err(err);
                            }

                            // Overwrite when invalid data is found
                            let mut buf = self.take_buf();
                            format::se(self.format, buf.as_mut(), &current)?;
                            self.file.set_len(0).await?;
                            tri!(self, self.file.write(buf));
                        }

                        current
                    }
                }
            };
            Ok(ret)
        } else {
            tri!(self, self.file.read(buf));
            format::des(self.format, &self.buf)
        }
    }
}

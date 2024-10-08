use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
};

use serde::{de::DeserializeOwned, Serialize};

use crate::{format, marker::Blocking, Io, VowResult};

impl<F: VowFile> Io<F, Blocking> {
    pub(crate) fn sync<T>(&mut self, current: Option<T>, overwrite: bool) -> VowResult<T>
    where
        T: Serialize + DeserializeOwned,
    {
        self.buf.clear();

        if let Some(mut current) = current {
            let ret = if overwrite {
                format::se(self.format, &mut self.buf, &current)?;
                self.file.write_all(&self.buf)?;
                current
            } else {
                self.file.read_to_end(&mut self.buf)?;
                match format::des(self.format, &self.buf) {
                    Ok(value) => value,
                    Err(err) => {
                        if err.is_invalid_data() {
                            if self.deny_invalid {
                                return Err(err);
                            }

                            // Overwrite when invalid data is found
                            current = self.sync(Some(current), true)?;
                        }
                        current
                    }
                }
            };
            Ok(ret)
        } else {
            self.file.read_to_end(&mut self.buf)?;
            format::des(self.format, &self.buf)
        }
    }
}

/// Low-level trait for synchronous file operations
pub trait VowFile: Read + Write {
    /// Open a new file at the given path
    fn open(path: impl AsRef<Path>) -> io::Result<Self>
    where
        Self: Sized;

    /// Set the length of the file
    fn set_len(&mut self, len: u64) -> io::Result<()>;
}

impl VowFile for File {
    fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
    }

    fn set_len(&mut self, len: u64) -> io::Result<()> {
        Self::set_len(self, len)
    }
}

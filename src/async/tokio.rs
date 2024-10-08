use std::path::Path;

use crate::{
    r#async::{BufFut, IoFut},
    VowFileAsync,
};

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

impl VowFileAsync for File {
    fn open(path: &Path) -> impl IoFut<Self>
    where
        Self: Sized,
    {
        async move {
            tokio::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .open(path)
                .await
        }
    }

    fn read(&mut self, mut buf: Vec<u8>) -> impl BufFut {
        async move {
            let res = AsyncReadExt::read_to_end(self, &mut buf).await.map(|_| ());
            (res, buf)
        }
    }

    fn write(&mut self, buf: Vec<u8>) -> impl BufFut {
        async move {
            let res = AsyncWriteExt::write_all(self, &buf).await;
            (res, buf)
        }
    }

    fn flush(&mut self) -> impl IoFut<()> {
        async move {
            AsyncWriteExt::flush(self).await?;
            Ok(())
        }
    }

    fn set_len(&mut self, len: u64) -> impl IoFut<()> {
        async move {
            #[allow(clippy::use_self)]
            File::set_len(self, len).await?;
            Ok(())
        }
    }
}

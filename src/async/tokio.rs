use std::{io::SeekFrom, path::Path};

use crate::{
    r#async::{BufFut, IoFut},
    VowFileAsync,
};

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
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
            self.seek(SeekFrom::Start(len)).await?;
            Self::set_len(self, len).await?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::VowFileAsync;
    use tokio::io::{AsyncReadExt, AsyncSeekExt};

    #[tokio::test]
    async fn test_write() {
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open("/tmp/tokio")
            .await
            .unwrap();
        let (res, _) =
            VowFileAsync::write(&mut file, b"{\"a\":43,\"b\":\"tokio!\"}".to_vec()).await;

        res.unwrap();

        let mut buf = vec![];

        file.seek(tokio::io::SeekFrom::Start(0)).await.unwrap();
        file.read_to_end(&mut buf).await.unwrap();

        assert_eq!(buf, b"{\"a\":43,\"b\":\"tokio!\"}");
    }
}

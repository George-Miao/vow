use async_std::{
    fs::{File, OpenOptions},
    io::{ReadExt, SeekExt, SeekFrom, WriteExt},
};
use std::path::Path;

use crate::VowFileAsync;

impl VowFileAsync for File {
    fn open(path: &Path) -> impl super::IoFut<Self>
    where
        Self: Sized,
    {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
    }

    fn read(&mut self, mut buf: Vec<u8>) -> impl super::BufFut {
        async move {
            let res = self.read_to_end(&mut buf).await.map(|_| ());
            (res, buf)
        }
    }

    fn write(&mut self, buf: Vec<u8>) -> impl super::BufFut {
        async move {
            let res = self.write_all(&buf).await;
            (res, buf)
        }
    }

    fn flush(&mut self) -> impl super::IoFut<()> {
        async move {
            <Self as WriteExt>::flush(self).await?;
            Ok(())
        }
    }

    fn set_len(&mut self, len: u64) -> impl super::IoFut<()> {
        async move {
            self.seek(SeekFrom::Start(len)).await?;
            Self::set_len(self, len).await?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::VowFileAsync;
    use async_std::io::ReadExt;

    #[async_std::test]
    async fn test_write() {
        let mut file = async_std::fs::File::create("/tmp/async-std").await.unwrap();
        let (res, _) =
            VowFileAsync::write(&mut file, b"{\"a\":43,\"b\":\"async std!\"}".to_vec()).await;

        res.unwrap();

        let mut buf = vec![];
        file.read_to_end(&mut buf).await.unwrap();

        assert_eq!(buf, b"{\"a\":43,\"b\":\"async std!\"}");
    }
}

use std::{io::Cursor, path::Path};

use crate::{
    r#async::{BufFut, IoFut},
    VowFileAsync,
};

use compio_fs::File;
use compio_io::{repeat, AsyncReadAtExt, AsyncReadExt, AsyncWriteAtExt};

impl VowFileAsync for File {
    fn open(path: &Path) -> impl IoFut<Self>
    where
        Self: Sized,
    {
        async move {
            compio_fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .open(path)
                .await
        }
    }

    fn read(&mut self, buf: Vec<u8>) -> impl BufFut {
        async move {
            let res = self.read_to_end_at(buf, 0).await;
            (res.0.map(|_| ()), res.1)
        }
    }

    fn write(&mut self, buf: Vec<u8>) -> impl BufFut {
        async move {
            let res = self.write_all_at(buf, 0).await;
            (res.0, res.1)
        }
    }

    fn flush(&mut self) -> impl IoFut<()> {
        async move {
            self.sync_data().await?;
            Ok(())
        }
    }

    fn set_len(&mut self, len: u64) -> impl IoFut<()> {
        async move {
            let curr_len = self.metadata().await?.len();

            if curr_len > len {
                let mut this = Cursor::new(self);
                this.set_position(len);
                compio_io::copy(&mut repeat(0).take(curr_len - len), &mut this).await?;
            }

            Ok(())
        }
    }
}

#[cfg(test)]
#[cfg(feature = "backend-compio")]
mod test {
    use compio_io::AsyncWriteAtExt;

    use crate::VowFileAsync;

    #[compio::test]
    async fn test_write() {
        let mut file = compio_fs::File::create("/tmp/test.txt").await.unwrap();
        file.write_all_at("12345678", 0).await.unwrap();
        let read = compio_fs::read("/tmp/test.txt").await.unwrap();
        assert_eq!(read, b"12345678");

        file.set_len(4).await.unwrap();
        let read = compio_fs::read("/tmp/test.txt").await.unwrap();
        assert_eq!(read, b"1234\0\0\0\0");
    }
}

# Vow

Vow is a simple but generic data binding library for Rust.

It allows you to bind any type that implements `serde::Serialize + DeserializeOwned` to a file, and keeps the file up-to-date while supporting multiple backends (both synchronous and asynchronous).

Currently [`tokio`](https://tokio.rs) and [`compio`](https://github.com/compio-rs/compio) are supported as asynchronous backends, and [`std::fs`](https://doc.rust-lang.org/std/fs/index.html) is supported as a synchronous backend.

## Example

```rust
use serde::{Deserialize, Serialize};
use vow::*;

#[derive(Serialize, Deserialize)]
struct MyData {
    a: i32,
    b: String,
}

#[compio::main]
async fn main() {
    let mut data = VowAsync::open_compio("/tmp/data.json")
        .default(MyData {
            a: 42,
            b: "hello".to_string(),
        })
        .overwrite_local()
        .build()
        .await
        .unwrap();

    assert_eq!(data.a, 42);
    data.update(|data| data.a += 1).await.unwrap();
    assert_eq!(data.a, 43);

    data.update(|data| data.b += " world!").await.unwrap();
    assert_eq!(data.b, "hello world!");

    data.map(|data| MyData {
        b: String::new(),
        ..data
    })
    .await
    .unwrap();
    assert_eq!(data.b, "");
}
```

For more examples, see the `examples` directory.

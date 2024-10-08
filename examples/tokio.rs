use serde::{Deserialize, Serialize};
use vow::*;

#[derive(Serialize, Deserialize)]
struct MyData {
    a: i32,
    b: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut data = VowAsync::open_tokio("/tmp/data.json")
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

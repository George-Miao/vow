use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyData {
    a: i32,
    b: String,
}

#[cfg(not(feature = "backend-tokio"))]
fn main() {
    panic!("This example requires the 'backend-tokio' feature.");
}

#[cfg(feature = "backend-tokio")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use vow::*;

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

    let content = std::fs::read_to_string("/tmp/data.json").unwrap();
    assert_eq!(content, r#"{"a":43,"b":"hello world!"}"#);
}

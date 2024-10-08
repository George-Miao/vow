use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyData {
    a: i32,
    b: String,
}

#[cfg(not(feature = "backend-async-std"))]
fn main() {
    panic!("This example requires the 'backend-async-std' feature.");
}

#[cfg(feature = "backend-async-std")]
#[async_std::main]
async fn main() {
    use vow::*;

    let mut data = VowAsync::open_async_std("/tmp/data.json")
        .default(MyData {
            a: 42,
            b: "async".to_string(),
        })
        .overwrite_local()
        .build()
        .await
        .unwrap();

    assert_eq!(data.a, 42);
    data.update(|data| data.a += 1).await.unwrap();
    assert_eq!(data.a, 43);

    data.update(|data| data.b += " std!").await.unwrap();
    assert_eq!(data.b, "async std!");

    data.update(|MyData { b, .. }| *b = "async std!".to_owned())
        .await
        .unwrap();
    assert_eq!(data.b, "async std!");

    data.flush().await.unwrap();

    let content = std::fs::read_to_string("/tmp/data.json").unwrap();
    assert_eq!(content, r#"{"a":43,"b":"async std!"}"#);
}

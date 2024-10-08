use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyData {
    a: i32,
    b: String,
}
#[cfg(not(feature = "backend-compio"))]
fn main() {
    panic!("This example requires the 'backend-compio' feature.");
}

#[cfg(feature = "backend-compio")]
#[compio::main]
async fn main() {
    use vow::*;

    let mut data = VowAsync::open_compio("/tmp/data.toml")
        .default(MyData {
            a: 42,
            b: "hello".to_string(),
        })
        .toml()
        .overwrite_local()
        .build()
        .await
        .unwrap();

    assert_eq!(data.a, 42);
    data.update(|data| data.a += 1).await.unwrap();
    assert_eq!(data.a, 43);

    data.update(|data| data.b += " world!").await.unwrap();
    assert_eq!(data.b, "hello world!");

    let content = std::fs::read_to_string("/tmp/data.toml").unwrap();
    assert_eq!(content, "a = 43\nb = \"hello world!\"\n");
}

use serde::{Deserialize, Serialize};
use vow::*;

#[derive(Serialize, Deserialize)]
struct MyData {
    a: i32,
    b: String,
}

fn main() {
    let mut data = Vow::open("/tmp/data.json")
        .default(MyData {
            a: 42,
            b: "hello".to_string(),
        })
        .overwrite_local()
        .build()
        .unwrap();

    assert_eq!(data.a, 42);
    data.update(|data| data.a += 1).unwrap();
    assert_eq!(data.a, 43);

    data.update(|data| data.b += " world!").unwrap();
    assert_eq!(data.b, "hello world!");

    data.map(|data| MyData {
        b: String::new(),
        ..data
    })
    .unwrap();
    assert_eq!(data.b, "");
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Foo {
    x: i32,
    y: i32,
}

fn main() {
    println!("Hello, Bazel from Rust!");
}

use rand::random;


fn main() {
    println!("Hello, Bazel from Rust!");
    let nr: u8 = random();

    println!("Here, a random number: {}", nr);
}

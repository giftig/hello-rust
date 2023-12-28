pub mod crab;
pub mod oop;
pub mod json;

fn main() {
    crab::say_slogan();
    println!("\n==========================================\n");

    oop::woof();
    println!("\n==========================================\n");

    json::read_json().unwrap();
    println!("\n==========================================\n");
}

// ⭐ Ownership and borrowing concepts seem important in Rust
// https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

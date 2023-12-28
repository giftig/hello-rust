pub mod crab;
pub mod oop;

use oop::basic::{Dog, Woofer};

fn main() {
    crab::say_slogan();
    println!("\n==========================================\n");
    oop::woof();
}

// ⭐ Ownership and borrowing concepts seem important in Rust
// https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

pub mod args;
pub mod crab;
pub mod oop;
pub mod json;

use args::{Args, DemoType};
use clap::Parser;

fn main() {
    let args = Args::parse();

    match args.demo {
        DemoType::All => all_demos(args),
        DemoType::CrabSay => crab::say_slogan(args.crab_phrase),
        DemoType::Json | DemoType::Books => json::read_json().unwrap(),
        DemoType::Woof => oop::woof(),
    }

}

fn all_demos(args: Args) -> () {
    crab::say_slogan(args.crab_phrase);
    println!("\n==========================================\n");

    oop::woof();
    println!("\n==========================================\n");

    json::read_json().unwrap();
    println!("\n==========================================\n");
}

// ⭐ Ownership and borrowing concepts seem important in Rust
// https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

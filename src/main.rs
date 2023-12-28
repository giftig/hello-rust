use ferris_says::say;
use std::io::{BufWriter, stdout};

fn main() {
    let stdout = stdout();
    let mut message = String::from("I'm a stupid shit-eating crab");

    // ❗ String is a mutable string created from the literal, str. We can append to the String
    // format! is how Rust does string interpolation.
    let num = an_integer();
    message.push_str(&format!(": {num}"));

    let width = message.chars().count();

    // 🔻 stdout.lock() prevents concurrent use of stdout; we're claiming access to the resource
    //    https://doc.rust-lang.org/beta/std/io/struct.StdoutLock.html
    let mut writer = BufWriter::new(stdout.lock());

    // 🔻 .unwrap is basically .get on a Scala Option or Try
    //    `say` is returning a Result<()>, akin to a Try[Unit] for io operations.
    // 🔗 Result: https://doc.rust-lang.org/nightly/std/io/type.Result.html
    // 🔗 say: https://docs.rs/ferris-says/latest/ferris_says/fn.say.html
    // ❗ The compiler will warn if you haven't handled a possible error
    say(&message, width, &mut writer).unwrap();
}

// 🔻 Compiler deliberately enforces snake case over camel case by providing a warning
fn an_integer() -> i32 {
    let num: i32 = 123;

    // ❗ N.B. semicolons in Rust essentially mean "discard the expression's value", returning
    //    () instead. While semicolons usually seem mandatory, here we must omit it
    num
}

// ⭐ Ownership and borrowing concepts seem important in Rust
// https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

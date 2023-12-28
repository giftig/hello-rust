pub mod basic;

use crate::{Dog, Woofer};

pub fn woof() {
    // We can collect a Vector of Woofers, anything with a typeclass providing the
    // Woofer capability to a type.
    let big_dog = Dog { name: String::from("Fido"), size: 10 };
    let lil_dog = Dog { name: String::from("Runt"), size: 3 };
    let other_woofer = String::from("disguised cat");

    // Rust uses the `dyn` keyword to indicate that we're referring to a trait rather
    // than a type, to avoid ambiguity. The actual type is not known at compile time.
    // https://doc.rust-lang.org/std/keyword.dyn.html
    let all_woofers: Vec<&dyn Woofer> = vec![&big_dog, &lil_dog, &other_woofer];

    for w in all_woofers.iter() {
        println!("{}", w.woof());
    }
}

// Rust has structs to encapsulate data
pub struct Dog {
    pub name: String,
    pub size: i32
}

// Rust traits define capabilities...
pub trait Woofer {
    fn woof(&self) -> String;
}

//  ...and then let you provide an `impl` as a typeclass for a given type.
impl Woofer for String {
    fn woof(&self) -> String {
        format!("{self}: woof")
    }
}

impl Woofer for Dog {
    fn woof(&self) -> String {
        format!("{}: wo{}f!", self.name, "o".repeat(self.size.try_into().unwrap()))
    }
}

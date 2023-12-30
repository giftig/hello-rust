// ⭐ Ownership and borrowing concepts are fundamental in Rust
// The following code is some experimentation with variations of the tutorial examples:
// https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

pub(super) fn ownership() -> () {
    // We're dealing with Strings as a simple example of data stored in heap
    move_demo();
    clone_demo();
    borrow_demo();
    borrow_mut_demo();
}

fn move_demo() -> () {
    let s1 = String::from("String in heap");

    // Rust will invalidate old references to the data when you create new ones,
    // so reassigning s1 will make the original s1 variable unusable:
    let s2 = s1;

    // This won't compile:
    //     println!("{}", s1)
    // ...and instead rust will suggest using .clone() to copy the heap data

    // Because a new reference is allocated on a function call, this will have the same problem:
    print_something(s2);
    // println!("{}", s2);

    // This is a *move* from s1 to s2 and then to s in the test fn, invalidating old references
    // One valid reference at once allows correct deallocation when that reference is descoped
}

fn clone_demo() -> () {
    let s1 = String::from("Another string");

    // Here we're explicitly cloning the string in heap. Obviously this is less efficient, but
    // now s1 and s2 are different pointers and separately deallocated.
    let s2 = s1.clone();

    println!("s1: '{}' ... s2: ({})", s1, s2);

    // Rust doesn't clone anything in heap automatically, we always have control over when to clone
}

fn borrow_demo() -> () {
    let s1 = String::from("Generous string");

    // To let another function use the value in a read-only way, without having to pass ownership
    // back and forth, we can create a &reference and pass that in instead.
    something_borrowed(&s1);

    // The original variable is still fine because we still own it, it hasn't moved.
    println!("{}", s1);

    // Rust also guards against allowing the owned reference to expire while it's being borrowed by
    // something else; it will consider a borrowed reference to live until its final use.
}

fn borrow_mut_demo() -> () {
    // You can allow a function to borrow and mutate a variable by making it mutable and creating a
    // mutable reference with mut explicitly
    let mut s1 = String::from("Mutable borrow");

    // Obviously this isn't good practice in any language, but Rust at least makes it very explicit
    // You also can't have any other borrowed references while a mutable reference is in use.
    ragify(&mut s1);

    println!("{}", s1);
}

fn print_something(s: String) {
    println!("{}", s);
}

fn something_borrowed(s: &String) {
    println!("{}", s);
}

fn ragify(s: &mut String) -> () {
    *s = s.to_uppercase() + "!";
}

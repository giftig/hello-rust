mod book;

use std::fs;

use serde_json::Result;

use book::Book;

pub(super) fn read_json() -> Result<()> {
    // 🔻 Note the raw string literal notation here: https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
    let huge_ships_raw = r#"
        {
            "title": "How to Avoid Huge Ships",
            "author": "Trimmer, John W.",
            "isbn": "978-0881000191"
        }
    "#;

    // 🔻 ? operator unwraps valid results, or else propagates errors to this function's Result
    let huge_ships: Book = serde_json::from_str(huge_ships_raw)?;
    println!("My favourite book is \"{}\" by {}", huge_ships.title, huge_ships.author);

    // 🔻 Reading a file to a string is very simple with stdlib, though serde can also read from a
    // Reader, e.g. a file: https://docs.rs/serde_json/latest/serde_json/fn.from_reader.html
    // 🔻 Note that the expect is kind of cheating here as I have a different type of error, so to
    // propagate either the io error or the JSON error I'd need to combine different Results
    let green_grass_raw = fs::read_to_string("books/book1.json").expect("Couldn't read file!");

    let green_grass: Book = serde_json::from_str(&green_grass_raw)?;
    println!("My second favourite is \"{}\" by {}", green_grass.title, green_grass.author);

    Ok(())
}

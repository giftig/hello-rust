mod book;

use serde_json::Result;

use book::Book;

pub(super) fn read_json() -> Result<()> {
    // 🔻 Note the raw string literal notation here: https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
    let raw = r#"
        {
            "title": "How to Avoid Huge Ships",
            "author": "Trimmer, John W.",
            "isbn": "978-0881000191"
        }
    "#;

    let book: Book = serde_json::from_str(raw)?;

    println!("My favourite book is \"{}\" by {}", book.title, book.author);

    Ok(())
}

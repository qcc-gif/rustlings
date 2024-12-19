<<<<<<< HEAD
fn main() {
    // DON'T EDIT THIS SOLUTION FILE!
    // It will be automatically filled after you finish the exercise.
=======
// Lifetimes are also needed when structs hold references.

struct Book<'a> {
    //     ^^^^ added a lifetime annotation
    author: &'a str,
    //       ^^
    title: &'a str,
    //      ^^
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
>>>>>>> 26cf4989a2d9ffe6464e737ec341305384c86080
}

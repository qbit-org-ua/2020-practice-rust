struct Book {
    title: String,
}

fn read_book_owned(book: Book) -> Book {
    println!("Reading Book \"{}\"", book.title);
    let another_book = Book {
        title: String::from("Qwe"),
    };
    return another_book;
}

fn read_book(book: &mut Book) {
    println!("Reading Book \"{}\"", book.title);
    book.title += "wqe";
}

fn main() {
    let mut harry_potter_book = Book {
        title: String::from("Harry Potter and Philosopher Stone"),
    };
    read_book(&mut harry_potter_book);
    read_book(&mut harry_potter_book);
    let mut book_from_friend = read_book_owned(harry_potter_book);
    read_book(&mut book_from_friend);
}

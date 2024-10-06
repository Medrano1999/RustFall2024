use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    let mut file = File::create(filename).unwrap();

    for book in books {
        // Writing each book's data as a comma-separated line
        write!(file, "{},{},{}\n", book.title, book.author, book.year).unwrap();
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut books: Vec<Book> = Vec::new(); 

    // Reading each line and splitting into parts
    for line in reader.lines() {
        let line = line.unwrap(); 
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            // Create a new book from the parts
            let title = parts[0].to_string();
            let author = parts[1].to_string();
            let year = parts[2].parse().unwrap();  // Convert year to u16

            books.push(Book { title, author, year });
        }
    }

    books  // Return the vector of books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}

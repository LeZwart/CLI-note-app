use std::io;
use std::fs;

fn main() {
    // Works exactly as 'while True'
    loop {
        print!("\n\n\nWelcome to the command line text editor!\n\n");
        println!("Which action do you want to perform?");
        println!("[A] Create new document");
        println!("[B] Open existing document");
        println!("[C] List documents");
        println!("[D] Edit document");
        println!("[E] Exit");
    
        // Read user input
        let input: String = readline().to_uppercase();
        let input = input.as_str();
    
        // 'match' is similar to 'switch' in other languages
        match input {
            "A" => create_document(),
            "B" => open_document(),
            "C" => list_documents(),
            "D" => edit_document(),
            "E" => exit(),
            _ => println!("Invalid action!"),
        }
    }
}

fn readline() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_string();
}

fn create_document() {
    println!("Enter the name of the new document:");
    let filename = readline();

    println!("Enter the contents of the new document:");
    let contents = readline();

    fs::write(filename.clone(), contents.clone())
        .expect("Something went wrong writing the file");

    println!("Created {} with contents:\n{}", filename, contents);
    readline();
}

fn open_document() {
    println!("Enter the name of the document to open:");
    let filename = readline();

    let contents = fs::read_to_string(filename.clone())
        .expect("Something went wrong reading the file");

    println!("Opened {} with contents:\n{}", filename, contents);
    readline();
}

fn list_documents() {
    let paths = fs::read_dir(".")
        .expect("Failed to read directory");

    println!("Documents in current directory:");
    for path in paths {
        let path = path
            .expect("Failed to get path")
            .path();
        println!("{}", path.display());
    }
    readline();
}

fn edit_document() {
    println!("Enter the name of the document to edit:");
    let filename = readline();

    let contents = fs::read_to_string(filename.clone())
        .expect("Something went wrong reading the file");

    println!("Enter the new contents of the document:");
    let new_contents = readline();

    fs::write(filename.clone(), new_contents.clone())
        .expect("Something went wrong writing the file");

    println!("Edited {} with new contents:\n{}", filename, new_contents);
    readline();
}

fn exit() {
    println!("Exiting...");
    std::process::exit(0);
}


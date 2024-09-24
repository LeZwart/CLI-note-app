use std::io;
use std::fs;
use whoami;

fn get_document_path() -> String {
    return format!("C:/Users/{}/Documents/CLI_Notes_App", whoami::username());
}

fn main() {

    // Initialize document folder
    let document_path = get_document_path();
    fs::create_dir_all(document_path.clone())
        .expect("Failed to create document folder");


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
    // Ask for filename
    println!("Enter the name of the new document:");
    let filename = readline();

    // Ask for contents
    println!("Enter the contents of the new document:");
    let contents = readline();

    // Write to file
    let path = format!("{}/{}.txt", get_document_path(), filename);
    fs::write(path, contents.clone())
        .expect("Something went wrong writing the file");

    // Print success message
    println!("Created {} with contents:\n{}", filename, contents);
    readline();
}

fn open_document() {
    println!("Enter the name of the document to open:");
    let filename = readline();

    match fs::read_to_string(format!("{}/{}.txt", get_document_path(), filename)) {
        Ok(contents) => {
            println!("\nContents of {}:", filename);
            println!("{}", contents);
        },
        Err(_) => {
            println!("{} does not exist!", filename);
        }
        
    }
    readline();
}

fn list_documents() {
    let paths = fs::read_dir(get_document_path())
        .expect("Failed to read directory");

    println!("Saved Documents:");
    for path in paths {
        let path = path
            .expect("Failed to get path")
            .path();

        // Only print .txt files
        if path.extension().unwrap() == "txt" {
            println!("{}", path.file_stem().unwrap().to_str().unwrap());
        }
    }
    readline();
}

// TODO: Make more user friendly
fn edit_document() {
    println!("Enter the name of the document to edit:");
    let filename = readline();

    // Check if file exists
    match fs::read_to_string(format!("{}/{}.txt", get_document_path(), filename)) {

        // If file exists, read contents, print them, ask for new content, write to file
        Ok(mut contents) => {
            println!("Contents of {}:", filename);
            println!("{}", contents);

            println!("Enter new contents:");
            let new_contents = readline();
            contents = new_contents.clone();

            fs::write(format!("{}/{}.txt", get_document_path(), filename), contents)
                .expect("Error while writing to file");

            println!("Edited {} with new contents:\n{}", filename, new_contents);
        },
        Err(_) => {
            println!("{} does not exist!", filename);
        }
        
    }
}

fn exit() {
    println!("Exiting...");
    std::process::exit(0);
}


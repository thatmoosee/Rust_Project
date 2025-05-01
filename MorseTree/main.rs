// Morse Tree
// By Max Quirk and Josh Schmidt
// This program decodes Morse code using a binary tree, reading mappings from letters.txt
// and allowing users to input Morse code sequences for decoding.

// Import standard library modules for environment variables, file handling, and I/O
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

// Include the morse_tree module from morse_tree.rs
mod morse_tree;
// Import the MorseTree struct for use in this file
use morse_tree::MorseTree;

// Main function: entry point of the program
fn main() {
    // Initialize an array of 10 integers, setting indices 1-9 to values 1-9
    // This array is unused in the current logic but mirrors the original Java code
    let mut nums = [0; 10];
    for i in 1..10 {
        nums[i] = i as i32;
    }

    // Create a new MorseTree instance to store Morse code mappings
    let mut tree = MorseTree::new();
    // Build the tree by reading mappings from letters.txt
    build_tree(&mut tree);
    // Print the result of 3/2 (integer division, outputs 1), mirroring Java code
    println!("{}", 3 / 2);

    // Initialize a reusable String buffer for user input
    let mut message = String::new();
    // Main loop: prompt user for Morse code input until they enter "stop"
    loop {
        // Prompt the user to enter a Morse code sequence or "stop" to exit
        print!("Please enter an encoded message or stop to exit: ");
        // Flush stdout to ensure the prompt appears before reading input
        io::stdout().flush().expect("Failed to flush stdout");

        // Clear the message buffer to avoid retaining old input
        message.clear();
        // Read a line of input from stdin into the message buffer
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");
        // Trim whitespace from the input to clean it up
        let message = message.trim();

        // Check if the user wants to exit by entering "stop" (case-insensitive)
        if message.to_lowercase() == "stop" {
            break;
        }
        // Validate input: ensure it contains at least one '.' or '-' (Morse code characters)
        if !message.chars().any(|c| c == '.' || c == '-') {
            println!("Please enter a valid Morse Code!");
        } else {
            // Decode the Morse code input using the tree and print the result
            let decoded = decode(message, &tree);
            println!("{}", decoded);
        }
    }
    // Print a farewell message when the program exits
    println!("Thank you for using our decoder!");
}

// Builds the Morse code tree by reading mappings from letters.txt
fn build_tree(tree: &mut MorseTree) {
    // Get the project root directory from CARGO_MANIFEST_DIR, default to current dir if unset
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| String::from("."));
    // Construct the path to letters.txt in the project root
    let file_path = format!("{}/letters.txt", manifest_dir);
    // Attempt to open letters.txt, with a fallback to the current directory
    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => {
            match File::open("letters.txt") {
                Ok(file) => file,
                Err(e) => {
                    // If the file can't be found in either location, print an error and exit
                    eprintln!("File not found at {} or current directory: {}", file_path, e);
                    std::process::exit(1);
                }
            }
        }
    };

    // Create a buffered reader for efficient line-by-line file reading
    let reader = BufReader::new(file);
    // Iterate over each line in the file
    for line in reader.lines() {
        // Handle potential errors reading a line, skip if error occurs
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        };
        // Split the line into Morse code entries (e.g., "e. t- i..")
        let entries: Vec<&str> = line.split_whitespace().collect();
        // Process each entry in the line
        for entry in entries {
            // Ensure the entry is at least 2 characters (letter + Morse code)
            if entry.len() < 2 {
                eprintln!("Invalid entry in file: {}", entry);
                continue;
            }
            // Split the entry into letter (first char) and Morse code (rest)
            let (letter, morse) = entry.split_at(1);
            // Format the entry as uppercase letter + Morse code (e.g., "E.")
            let formatted_entry = format!("{}{}", letter.to_uppercase(), morse);
            // Add the entry to the Morse tree
            tree.add(&formatted_entry);
        }
    }
}

// Decodes a Morse code message using the provided MorseTree
fn decode(message: &str, tree: &MorseTree) -> String {
    // Delegate decoding to the MorseTree's decode method
    tree.decode(message)
}
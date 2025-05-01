use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

mod morse_tree;
use morse_tree::MorseTree;

fn main() {
    let mut nums = [0; 10];
    for i in 1..10 {
        nums[i] = i as i32;
    }
    let mut tree = MorseTree::new();
    build_tree(&mut tree);
    println!("{}", 3 / 2);

    let mut message = String::new();
    loop {
        print!("Please enter an encoded message or stop to exit: ");
        io::stdout().flush().expect("Failed to flush stdout");

        message.clear();
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");
        let message = message.trim();

        if message.to_lowercase() == "stop" {
            break;
        }
        if !message.chars().any(|c| c == '.' || c == '-') {
            println!("Please enter a valid Morse Code!");
        } else {
            let decoded = decode(message, &tree);
            println!("{}",decoded);
        }
    }
    println!("Thank you for using our decoder!");
}

fn build_tree(tree: &mut MorseTree) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| String::from("."));
    let file_path = format!("{}/letters.txt", manifest_dir);
    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => {
            match File::open("letters.txt") {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("File not found at {} or current directory: {}", file_path, e);
                    std::process::exit(1);
                }
            }
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        };
        let entries: Vec<&str> = line.split_whitespace().collect();
        for entry in entries {
            if entry.len() < 2 {
                eprintln!("Invalid entry in file: {}", entry);
                continue;
            }
            let (letter, morse) = entry.split_at(1);
            let formatted_entry = format!("{}{}", letter.to_uppercase(), morse);
            tree.add(&formatted_entry);
        }
    }
}

fn decode(message: &str, tree: &MorseTree) -> String {
    tree.decode(message)
}
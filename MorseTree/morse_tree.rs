//Morse Tree implementation 
//By Max Quirk and Josh Schmidt


// Import the Display trait for formatting Node data in string output
use std::fmt::Display;

// Node struct: represents a node in the Morse code binary tree
// Generic type T allows flexibility, though used with String in MorseTree
#[derive(Debug)]
pub struct Node<T> {
    pub data: T,                    // Stores the data (e.g., a letter like "E" or empty string "")
    pub left: Option<Box<Node<T>>>, // Left child (navigated to by '.')
    pub right: Option<Box<Node<T>>>, // Right child (navigated to by '-')
}

// Implementation of Node methods for types that implement Display
impl<T: Display> Node<T> {
    // Creates a new Node with the given data and no children
    pub fn new(data: T) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }
}

// Implementation of the Display trait for Node to format its data
impl<T: Display> Display for Node<T> {
    // Formats the Node by writing its data to the formatter
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

// MorseTree struct: represents the binary tree for Morse code decoding
pub struct MorseTree {
    pub root: Option<Box<Node<String>>>, // Root node of the tree, initialized with empty data
}

// Implementation of MorseTree methods
#[allow(dead_code)] // Suppress warnings for unused methods (from_node, with_children)
impl MorseTree {
    // Creates a new MorseTree with an empty root node (data = "")
    pub fn new() -> Self {
        MorseTree {
            root: Some(Box::new(Node::new(String::new()))),
        }
    }

    // Creates a MorseTree from an existing Node<String>
    // Useful for constructing a tree with a predefined root
    pub fn from_node(node: Node<String>) -> Self {
        MorseTree {
            root: Some(Box::new(node)),
        }
    }

    // Creates a MorseTree with a root node containing the given data
    // and optional left and right subtrees
    pub fn with_children(data: String, left_tree: Option<MorseTree>, right_tree: Option<MorseTree>) -> Self {
        let mut node = Node::new(data); // Create root node with specified data
        // Assign left subtree’s root as the left child, if provided
        if let Some(left) = left_tree {
            node.left = left.root;
        }
        // Assign right subtree’s root as the right child, if provided
        if let Some(right) = right_tree {
            node.right = right.root;
        }
        MorseTree {
            root: Some(Box::new(node)),
        }
    }

    // Adds a Morse code mapping to the tree (e.g., "E." for letter E at path .)
    pub fn add(&mut self, message: &str) -> bool {
        // Return false if the input is empty (invalid mapping)
        if message.is_empty() {
            return false;
        }
        // Extract the letter (first character) and Morse code (rest)
        let letter = message.chars().next().unwrap().to_string();
        let morse = &message[1..];
        // Take the current root, update it with the new mapping, and restore it
        if let Some(root) = self.root.take() {
            self.root = Some(add_recursive(&letter, morse, root));
            true
        } else {
            // This case shouldn’t occur since root is initialized in new()
            false
        }
    }

    // Decodes a space-separated Morse code string (e.g., ". - .." to "ETU")
    pub fn decode(&self, morse_code: &str) -> String {
        // Split input into individual Morse code segments (e.g., [".", "-", ".."])
        let codes: Vec<&str> = morse_code.split_whitespace().collect();
        let mut result = String::new();
        
        // Decode each segment and append to the result
        for code in codes {
            let decoded = decode_recursive(String::new(), code, &self.root);
            // Return error if any segment is invalid
            if decoded == "Please enter a valid Morse Code!" {
                return decoded;
            }
            result.push_str(&decoded);
        }
        
        // Return error if no valid codes were decoded, otherwise return the result
        if result.is_empty() {
            "Please enter a valid Morse Code!".to_string()
        } else {
            result
        }
    }
}

// Recursive helper function to add a letter to the tree at the specified Morse code path
fn add_recursive(
    letter: &str,                    // The letter to add (e.g., "E")
    morse: &str,                    // The Morse code path (e.g., ".")
    mut node: Box<Node<String>>,    // The current node being processed
) -> Box<Node<String>> {
    // If the Morse code is empty, set the node’s data to the letter
    if morse.is_empty() {
        node.data = letter.to_string();
        node
    } else if morse.starts_with('.') {
        // Navigate left for '.', create an empty node if none exists
        let left = node.left.take().unwrap_or_else(|| Box::new(Node::new(String::new())));
        node.left = Some(add_recursive(letter, &morse[1..], left));
        node
    } else {
        // Navigate right for '-', create an empty node if none exists
        let right = node.right.take().unwrap_or_else(|| Box::new(Node::new(String::new())));
        node.right = Some(add_recursive(letter, &morse[1..], right));
        node
    }
}

// Recursive helper function to decode a single Morse code segment
fn decode_recursive(
    message: String,                        // Unused accumulator (kept for compatibility)
    morse_code: &str,                      // The Morse code segment to decode (e.g., ".")
    local_root: &Option<Box<Node<String>>>, // The current node in the tree
) -> String {
    match local_root {
        // Return error if the node is None (invalid path)
        None => "Please enter a valid Morse Code!".to_string(),
        Some(node) => {
            // If the Morse code is empty, return the node’s data (if non-empty)
            if morse_code.is_empty() {
                if node.data.is_empty() {
                    "Please enter a valid Morse Code!".to_string()
                } else {
                    node.data.clone()
                }
            } else {
                // Get the first character of the Morse code
                let first_char = morse_code.chars().next().unwrap();
                // Return error if the character is not '.' or '-'
                if first_char != '.' && first_char != '-' {
                    return "Please enter a valid Morse Code!".to_string();
                }
                // Navigate left for '.' or right for '-'
                if first_char == '.' {
                    decode_recursive(message, &morse_code[1..], &node.left)
                } else {
                    decode_recursive(message, &morse_code[1..], &node.right)
                }
            }
        }
    }
}
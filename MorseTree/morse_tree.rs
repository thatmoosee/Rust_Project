use std::fmt::Display;

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T: Display> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

pub struct MorseTree {
    pub root: Option<Box<Node<String>>>,
}

#[allow(dead_code)]
impl MorseTree {
    pub fn new() -> Self {
        MorseTree {
            root: Some(Box::new(Node::new(String::new()))),
        }
    }

    pub fn from_node(node: Node<String>) -> Self {
        MorseTree {
            root: Some(Box::new(node)),
        }
    }

    pub fn with_children(data: String, left_tree: Option<MorseTree>, right_tree: Option<MorseTree>) -> Self {
        let mut node = Node::new(data);
        if let Some(left) = left_tree {
            node.left = left.root;
        }
        if let Some(right) = right_tree {
            node.right = right.root;
        }
        MorseTree {
            root: Some(Box::new(node)),
        }
    }

    pub fn add(&mut self, message: &str) -> bool {
        if message.is_empty() {
            return false;
        }
        let letter = message.chars().next().unwrap().to_string();
        let morse = &message[1..];
        if let Some(root) = self.root.take() {
            self.root = Some(add_recursive(&letter, morse, root));
            true
        } else {
            false
        }
    }

    pub fn decode(&self, morse_code: &str) -> String {
        // Split input on spaces to handle multiple letters
        let codes: Vec<&str> = morse_code.split_whitespace().collect();
        let mut result = String::new();
        
        for code in codes {
            // Decode each Morse code segment from the root
            let decoded = decode_recursive(String::new(), code, &self.root);
            if decoded == "Please enter a valid Morse Code!" {
                return decoded;
            }
            result.push_str(&decoded);
        }
        
        if result.is_empty() {
            "Please enter a valid Morse Code!".to_string()
        } else {
            result
        }
    }
}

fn add_recursive(
    letter: &str,
    morse: &str,
    mut node: Box<Node<String>>,
) -> Box<Node<String>> {
    if morse.is_empty() {
        node.data = letter.to_string();
        node
    } else if morse.starts_with('.') {
        let left = node.left.take().unwrap_or_else(|| Box::new(Node::new(String::new())));
        node.left = Some(add_recursive(letter, &morse[1..], left));
        node
    } else {
        let right = node.right.take().unwrap_or_else(|| Box::new(Node::new(String::new())));
        node.right = Some(add_recursive(letter, &morse[1..], right));
        node
    }
}

fn decode_recursive(
    message: String,
    morse_code: &str,
    local_root: &Option<Box<Node<String>>>,
) -> String {
    match local_root {
        None => "Please enter a valid Morse Code!".to_string(),
        Some(node) => {
            if morse_code.is_empty() {
                // Only return the node's data (skip empty strings from non-leaf nodes)
                if node.data.is_empty() {
                    "Please enter a valid Morse Code!".to_string()
                } else {
                    node.data.clone()
                }
            } else {
                let first_char = morse_code.chars().next().unwrap();
                if first_char != '.' && first_char != '-' {
                    return "Please enter a valid Morse Code!".to_string();
                }
                if first_char == '.' {
                    decode_recursive(message, &morse_code[1..], &node.left)
                } else {
                    decode_recursive(message, &morse_code[1..], &node.right)
                }
            }
        }
    }
}
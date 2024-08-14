fn clone_and_modify(s: &String) -> String {
    // Clone the original string
    let mut cloned_string = s.clone();
    // Modify the cloned string by appending a new word
    cloned_string.push_str("World!");
    cloned_string
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}

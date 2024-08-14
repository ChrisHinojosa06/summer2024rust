fn concat_strings(s1: &String, s2: &String) -> String {
    // Create a new String and concatenate s1 and s2
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}
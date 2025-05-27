/// Міняє регістр символів у рядку:
/// якщо буква велика — перетворити на малу, якщо мала — на велику.
pub fn swap_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

// Тестування функції
fn main() {
    let text = "Hello, Rust!";
    let swapped = swap_case(text);
    println!("Original: {}", text);
    println!("Swapped : {}", swapped);
}

fn main() {
    let original_value = 42;
    
    // Pass by value (move) to a function and receive a new value.
    let new_value = modify_value(original_value);
    
    println!("Original value: {}", original_value); // This would result in an error.
    println!("New value: {}", new_value);
}

fn modify_value(input: i32) -> i32 {
    // Modify the input and return it
    input + 10
}

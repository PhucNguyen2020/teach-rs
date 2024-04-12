//! You can't change anything except adding or removing references.

fn main() {
    let mut data = "Rust is great!".to_string();

    get_char(&data); // Pass a reference

    string_uppercase(&mut data);  // Pass a mutable reference 
}

// Should not take ownership (accepts a reference)
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership (accepts a mutable reference)
fn string_uppercase(data: &mut String) {
    *data = data.to_uppercase(); // Modify the String in-place

    println!("{}", data);
}
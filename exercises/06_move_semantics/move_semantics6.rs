// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    // borrow the string (input type is a reference)
    get_char(&data);

    // take ownership of the input variable
    string_uppercase(data);
}

// Should not take ownership
// set the input type to &BorrowedString
fn get_char(data: &String) -> char {
    // return the actual unwrapped string (not the reference)
    data.chars().last().unwrap()
}

// Should take ownership
// remove the reference to the borrow String (&String)
fn string_uppercase(mut data: String) {
    // the input type is mutable so remove the reference (not borrowing we have ownership)
    data = data.to_uppercase();

    println!("{}", data);
}

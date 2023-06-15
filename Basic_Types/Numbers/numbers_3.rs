
// Modify `assert_eq!` to make it work
fn main() {
    let x = 5;
    // This will not work because default typ eis i32 and we are checking for u32
    //assert_eq!("u32".to_string(), type_of(&x));

    //so change the type to i32 from u32 an dit will work
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

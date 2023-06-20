/*

// Fix all errors without adding newline
fn main() {
    let s = String::from("hello");
    s.push(',');
    s.push(" world");
    s += "!".to_string();

    println!("{}", s);
}
*/


// Fix all errors without adding newline
fn main() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s.push_str("!");

    println!("{}", s);
}

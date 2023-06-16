/*
// Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}
*/

// Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello"); // String to &str
    let y = x; // instead clone copy
    println!("{:?}, {:?}", x, y);
}

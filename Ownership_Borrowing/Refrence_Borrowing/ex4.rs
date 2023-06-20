/*
// Fix error
fn main() {
    let mut s = String::from("hello, ");

    push_str(s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}*/


// Fix error
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s);  // pass mutable refrence

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

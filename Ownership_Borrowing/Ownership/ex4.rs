/*
// Fix the error without removing code line
fn main() {
    let s = String::from("hello, world");

    print_str(s);

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}*/

// Fix the error without removing code line
fn main() {
    let s = String::from("hello, world");

    print_str(s.clone()); // deep copy no moving

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}


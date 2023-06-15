// Fix the error below with least amount of modification
fn main() {
    //this will give error
    //let ( x, y) = (1, 2);
     
    // making x mutable solves this error as by default variables are immutable in rust
    let (mut x, y) = (1, 2); 
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
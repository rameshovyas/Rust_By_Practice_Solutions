
/*fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}*/
// ********** All codes Working Below  Just un commend and use as required **************//
// First approach is remove the moved variable here 'x'

/*fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x;
    println!("{}",y);
}*/

//second approach deep copy instead shallow using clone()

fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);
}

// Third initialize individually

/*fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = String::from("hello, world");
    println!("{},{}",x,y);
}*/


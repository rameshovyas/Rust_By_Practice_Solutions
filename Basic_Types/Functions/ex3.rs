/*
// Solve it in two ways
// DON'T let `println!` works
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    
}
*/

//First solution 

fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    //Used panic macro for this diverging function
    panic!("This will never return");
}


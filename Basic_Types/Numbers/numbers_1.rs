
// Remove something to make it work
fn main() {
    let x: i32 = 5;
    /* 
    //This will not work because types are different
    let mut y: u32 = 5;
    y = x; 
    
    // this will show warning of unused
    let z = 10; // Type of z ? 
    */

    //The solution is 
    let _y = x;
    
    let _z = 10; // Type of z ? 


    println!("Success!");
}

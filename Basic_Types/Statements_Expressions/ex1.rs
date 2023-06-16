// Make it work with two ways
/*fn main() {
    let v = {
        let mut x = 1;
        x += 2
    };
 
    assert_eq!(v, 3);
 
    println!("Success!");
 }*/

 //First way 
 // Make it work with two ways
/*fn main() {
    let v: i32 = {
        let mut x = 1;
        x +=2;  //Place a semi colon beacuse it is a statement 
        x       // return here
    };
 
    assert_eq!(v, 3);
 
    println!("Success!");
 }*/
 
 //Second way
 // Change 3 to () in asset_eq as we are not returning becuase x+=2 is x = x+2;
 // Make it work with two ways
fn main() {
    let v = {
        let mut x = 1;
        x += 2
    };
 
    assert_eq!(v, ());
 
    println!("Success!");
 }
 
 
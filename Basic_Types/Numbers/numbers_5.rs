// Fix errors and panics to make it work
/*fn main() {
    let v1 = 251_u8 + 8;
    let v2 = i8::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
 }*/
 
fn main() {
    let v1 = 251_u16 + 8;  // u8 will overflow so made it u16 (251+8 = 259 which is > 255)
    let v2 = u16::checked_add(251, 8).unwrap(); //made u8 to u16 to overcome overflow
    println!("{},{}",v1,v2);
 }
 
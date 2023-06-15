//Make it work in two distinct ways
/*fn main() {
    assert!(0.1+0.2==0.3);

    println!("Success!");
}
*/

//First Way Provide the type because default is f64

/*fn main() {
    assert!(0.1_f32+0.2_f32==0.3_f32);

    println!("Success!");
}
*/

//Second way we can use 'as' keyword

fn main() {
    assert!(0.1 as f32+0.2 as f32==0.3 as f32);

    println!("Success!");
}

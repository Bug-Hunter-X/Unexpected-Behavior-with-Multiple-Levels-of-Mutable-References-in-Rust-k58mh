fn main() {    let mut x = 5;    let ref_x = &mut x;    let ref_ref_x = &ref_x;    *ref_ref_x = 10;    println!("x = {}", x); // Output: x = 10}
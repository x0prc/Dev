//using the as keyword
fn main() {
    let decimal = 95.223_f32;
    let integer : i32 = decimal as i32;
    let c1: char = decimal as char;
    let c2: char = 'a';

    assert_eq!(integer, '95' as u8);
    println!("working");
}
// raw pointers to mem add
let p1: *mut i32 = values.as_mut_ptr();

// using from and into
let my_str = "no";
let string1 = String::from(my_str);
let string2 = my_str.to_string();

// impl for custom types
impl From<i32> for Number

// can also use tryfrom and tryinto to return a result.
// ToString any type to String

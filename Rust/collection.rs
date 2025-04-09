fn main() {
        let mut s: String = "hello:";
        s.push_str(" world!".to_string());
        s.push();

        move_ownership(s);
        assert_eq!(s, "hello: world!");
        println!("Success!");
}

//slice
fn main() {
    let mut s = String::from("hello, world");

    let slice1: &str = &s[0..5];
    let slice2: &str = &s[7..12];

    println!("{} {}", slice1, slice2);
}

// explicitly using utf8
use utf8 slice;
fn main() {
    let s = "The 🚀 goes to the moon! ";
    let rocket = utf8_slice::slice(s, 4, 5);
}

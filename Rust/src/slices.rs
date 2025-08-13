// sol : slices

fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    let s1: &[i32] = &arr[0..2]; // &[1, 2]

    let s2: &str = "hello, world";

    println!("Success!");
}

// holds 16 bytes
fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice : &[char] = &arr[..2];
    
    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}

// string slices
fn main() {
    let s: String = String::from("hello");

    let slice1: &str = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);

    println!("Success!");
}

// &string to &str implicit conversion
fn main() {
    let mut s: String = String::from("hello world");

    // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`. 
    let letter: &str = first_letter(&s);
    println!("the first letter is: {}", letter);
    s.clear(); // error!

    
}
fn first_letter(s: &str) -> &str {
    &s[..1]
}
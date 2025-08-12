// file sol : course.practice.rs

// Don't use clone ,use copy instead
fn main() {
    let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    let y: (i32, i32, (), &str) = x;
    println!("{:?}, {:?}", x, y);
}

// make the necessary variable mutable
fn main() {
    let s: String = String::from("Hello ");
    
    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}

// make the necessary variable mutable
fn main() {
    let s: String = String::from("Hello ");
    
    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}



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

fn main() {
   let t: (String, String) = (String::from("hello"), String::from("world"));

   let _s: String = t.0;

   // Modify this line only, don't use `_s`
   println!("{:?}", t.1);
}

fn main() {
   let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}


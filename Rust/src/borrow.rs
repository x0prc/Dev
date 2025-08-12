// sol : reference and borrowing

fn main() {
   let x: i32 = 5;
   // Fill the blank
   let p: &i32 = &x;

   println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

fn main() {
    let x: i32 = 5;
    let y: &i32 = &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success!");
}

fn main() {
    let mut s: String = String::from("hello, ");

    // Fill the blank to make it work
    let p: &mut String = &mut s;
    
    p.push_str("world");

    println!("Success!");
}

fn main() {
    let mut s: String = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}

fn main() {
    let mut s: String = String::from("hello, ");

    let r1: &mut String = &mut s;
    r1.push_str("world");
    let r2: &mut String = &mut s;
    r2.push_str("!");
    
    println!("{}",r2);
}


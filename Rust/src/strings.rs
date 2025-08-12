// sol : strings
fn main() {
    let s: &str = "hello, world";

    println!("Success!");
}

fn main() {
    let s: &str = "hello, world";
    greetings(&s)
}

fn greetings(s: &str) {
    println!("{}",s)
}

fn main() {
    let s1: String = String::from("hello,");
    let s2: String = String::from("world!");
    let s3: String = s1 + &s2; 
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}

fn main() {
    let s: &str = "hello, world";
    greetings(s.to_owned())
}

fn greetings(s: String) {
    println!("{}", s)
}

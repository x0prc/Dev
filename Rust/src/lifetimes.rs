// sol : lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x: &str = "long";
    let y: &str = "longer";
    
    println!("{}", longest(x, y));
}

fn invalid_output<'a>(s: &'a str) -> &'a str {
    s
}

fn main() {
    let s: String = String::from("foo");
    
    let x = invalid_output(&s);
    
    println!("{}", x);
}
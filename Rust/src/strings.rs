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

fn main() {
    // You can use escapes to write bytes by their hexadecimal values
    // Fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

// UTF8 string
fn main() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}


//slices
fn first(s: &String)-> usize {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    bytes.len()
}
let s = String::from("hello world");
let hello = &s[0..5];

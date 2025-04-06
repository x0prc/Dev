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
assert_eq!(slice, &[2, 3]);

//struct
#[derive(Deserialize, Debug)]
struct User {
    active: bool,
    email: String,
    sign_in_count: u64,
}

fn new(){
    let user1 = User {
        active: true,
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}

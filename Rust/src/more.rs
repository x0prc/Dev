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

// functions
fn add(a: i32, b: i32) -> i32 {
    a + b
}
let x = add(1,1);
let y = add(3,0);
let z = add(x,5);

//macros
println!("Hello!");
println!("{:?}", life);
println!("{:?}, {:?}", life, life); // for debugging

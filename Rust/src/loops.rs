//loop
let mut a = 0;
loop {
    if a == 5 {
        break;
    }
    println!("{:?}", a);
    a = a + 1;
}

//while
let mut a = 0;
while a != 5 {
    println!("{:?}", a);
    a = a + 1;
}

//match expression (checked by compiler everytime for every condition.)
fn main() {
    let some_bool = true;
    match some_bool {
        true => println!("It's true!"),
        false => println!("It's false!"),
    }
}

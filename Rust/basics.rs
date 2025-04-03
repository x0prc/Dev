use std::io;

fn main() {
    let mut x = 5;
    println!("Value of x is: {x}");
    x = 6;
    println!("Value of x is: {x}");
}
fn boolean() {
    let t = true;
    let f: bool = false;
}
// destructuring
fn destruct() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("value of y is {y}");
}
//array
fn arrays() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
//get array index from user
fn input() {
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("index was not a number");

    let element = a[index];
}
//parameter declarations
fn paramdec() {
    print_labeled_measurement(5, 'h');

    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("measurement is: {value}{unit_label}");
    }
}

//return type after ->
fn five() -> i32 {
    5
}

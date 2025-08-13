// sol : arrays
fn main() {
    // Fill the blank with proper array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 5);

    println!("Success!");
}

fn main() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    
    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);

    println!("Success!");
}

// 1-100 elements in array
fn main() {
    // Fill the blank
    let list: [i32; 100] = [1; 100] ;

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}

// elements must be of the same type.
fn main() {
    // Fix the error
    let _arr : [i32; 3] = [1, 2, 3];

    println!("Success!");
}

// panics if out of bound
fn main() {
    let names: [String; 2] = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<&T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // But indexing is not safe
    let _name1 = &names[1];

    println!("Success!");
}

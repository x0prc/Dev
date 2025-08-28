// sol : closures

fn main() {
    let x = 1;
    let closure = |val| val + x;
    assert_eq!(closure(2), 3);
}

// capture
fn main() {
    let color = String::from("green");

    let print = || println!("`color`: {}", color);

    print();
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`. 
    let _reborrow: &String = &color;

    println!("{}",color);
}

fn main() {
     let movable: Box<i32> = Box::new(3);

     let consume = || {
         println!("`movable`: {:?}", movable);
         take(movable);
     };

     consume();
     consume();
}

fn take<T>(_v: T) {}

// type inferred
fn main() {
    let example_closure = |x: String| -> String { x };

    let s: String = example_closure(String::from("hello"));

    /* Make it work, only change the following line */
    let n: String = example_closure(5.to_string());
}


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

// string literals
fn main() {
    let mut s: String = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}",s);
}

/* Fill in the blank */
fn exec<'a, F: FnMut(&'a str)>(mut f: F)  {
    f("hello")
}

// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
    F: FnOnce() {

    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting: &str = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell: String = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}


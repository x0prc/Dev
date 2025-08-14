// sol : pattern match

fn main() {
    let boolean: bool = true;

    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary: u8 = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

    println!("Success!");
}


enum MyEnum {
    Foo,
    Bar
}

fn main() {
    let mut count = 0;

    let v: Vec<MyEnum> = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) { // Fix the error by changing only this line
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}

// if let
fn main() {
    let o = Some(7);
    
    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
        
        println!("Success!");
    }
}

// if let and match
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let a: Foo = Foo::Qux(10);

    // Remove the codes below, using `match` instead 
    if let Foo::Bar = a {
        
    } else if let Foo::Baz = a {
        println!("match foo::baz")
    } else {
        println!("match others")
    }
    
    match a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("math foo::baz"),
        _ => println!("match others"),
    }
}


// shadowing
fn main() {
    let age = Some(30);
    if let Some(age) = age { // Create a new variable with the same name as previous `age`
       assert_eq!(age, 30);
    } // The new variable `age` goes out of scope here
    
    match age {
        // Match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
 }
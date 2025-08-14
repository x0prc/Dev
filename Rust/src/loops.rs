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
// conditionals
fn main() {
    let n: i32 = 5;

    let big_n: i32 =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            10 * n
        } else {
            println!(", and is a big number, halve the number");

            n / 2.0 as i32
        };

    println!("{} -> {}", n, big_n);
} 




// Fix the errors without adding or removing lines
fn main() {
    let names: [String; 2] = [String::from("liming"),String::from("hanmeimei")];
    for name in &names {
        // Do something with name...
        println!("{}", name);
    }

    println!("{:?}", names);

    let numbers: [i32; 3] = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        // Do something with n...
        println!("{}", n)
    }
    
    println!("{:?}", numbers);
} 

// continue
fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n != 66 {
           n+=1;
           continue;
       }
       
       break;
    }

    assert_eq!(n, 66);

    println!("Success!");
}

// use break to return
fn main() {
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}

// nested loops
fn main() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; // `break` is also works.
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }

            // This will continue the outer loop
            continue 'outer;
        }
    }

    assert!(count == 30);

    println!("Success!");
}
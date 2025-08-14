enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn which_way(go: Direction) {
    match go {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
        Direction::Left => println!("Left"),
        Direction::Right => println!("Right"),
    }
}

//struct
struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

let tall = my_box.height;
println!("the box is {:?}", tall);

//enum and struct together
enum Flavor {
    Apple,
    Banana,
    Cherry,
}
struct Drink {
    flavor: Flavor,
    fluid_oz: f64,

}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Apple => println!("Apple"),
        Flavor::Banana => println!("Banana"),
        Flavor::Cherry => println!("Cherry"),
    }

    println!("oz: {:?}, drink", drink.fluid_oz);
}

fn main() {
    let drink = Drink {
        flavor: Flavor::Apple,
        fluid_oz: 12.0,
    };
    print_drink(drink);
}

// pattern matching
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move{x: 1, y: 1};

    if let Message::Move{x: a, y: b} = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success!");
} 


[#derive(Debug)]
// Fill in the blank and fix the errors
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
} 

fn show_message(msg: Message) {
    println!("{:?}", msg);
}
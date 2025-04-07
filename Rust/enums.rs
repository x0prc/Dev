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

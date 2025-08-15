// methods 
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Complete the area method which return the area of a Rectangle.
    fn area(self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1: Rectangle = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);

    println!("Success!");
}

#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(self: &Self)  {
        println!("the current state is {}", self.color);
    }
    
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}
fn main() {
    println!("Success!");
}

// multiple impl
struct Rectangle {
    width: u32,
    height: u32,
}

// Using multiple `impl` blocks to rewrite the code below.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    println!("Success!");
}

// impl methods for enums
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color(&self) -> &str {
        match self {
            Self::Yellow => "yellow",
            Self::Red => "red",
            Self::Green => "green",
        }
    }
}

fn main() {
    let c: TrafficLightColor = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}

struct A;          // Concrete type `A`.
struct S(A);       // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn main() {
    // Using the non-generic functions
    reg_fn(S(A));          // Concrete type.
    gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(7)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('A'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen(7.7));

    println!("Success!");
}

struct Sheep {
    naked: bool,
    name: String,
}

trait Animal {
    fn new(name: String) -> Self;
    fn name(&self) -> &String;
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn new(name: String) -> Self {
        Sheep { naked: false, name }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn noise(&self) -> String {
        if self.naked {
            "baaaaah?".to_string()
        } else {
            "baaaaah!".to_string()
        }
    }
}
// trait objects
trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Duck is swimming");
    }
}
impl Bird for Duck {
    fn quack(&self) -> String {
        "quack!".to_string()
    }
}

//using &dyn and <dyn>
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8:{}", *self)
    }
}
fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}


trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

struct Student {}
impl Hello for Student {
    fn say_something(&self) -> String {
        String::from("I'm a good student")
    }
}
struct Teacher {}
impl Hello for Teacher {
    fn say_hi(&self) -> String {
        String::from("Hi, I'm your new teacher")
    }
    
    fn say_something(&self) -> String{
        String::from("I'm not a bad teacher")
    }
}

// sol : traits

fn main() {
    let s: Student = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t: Teacher = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!");
}

// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}


#[derive(Debug, PartialEq, PartialOrd)]
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    println!("One second looks like: {:?}", _one_second);
    let _this_is_true = (_one_second == _one_second);
    let _this_is_false = (_one_second > _one_second);

    let foot: Inches = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter: Centimeters = Centimeters(100.0);

    let cmp: &str =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}

// Box <dyn> usage

trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}

fn main() {
    // FILL in the blank.
    let duck : Duck = Duck;
    duck.swim();

    let bird: Box<dyn Bird> = hatch_a_bird(2);
    // This bird has forgotten how to swim, so below line will cause an error.
    // bird.swim();
    // But it can quak.
    assert_eq!(bird.quack(), "duck duck");

    let bird: Box<dyn Bird> = hatch_a_bird(1);
    // This bird has forgotten how to fly, so below line will cause an error.
    // bird.fly();
    // But it can quak too.
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");
}   

// IMPLEMENT this function.
fn hatch_a_bird(species : u8) -> Box<dyn Bird> {
    match species {
        1 => Box::new(Swan),
        2 => Box::new(Duck),
        _ => panic!(),
    }
}

// static and dynamic dispatch
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// IMPLEMENT below with generics.
fn static_dispatch<T: Foo>(a: T) {
    a.method();
}

// Implement below with trait objects.
fn dynamic_dispatch(a: &dyn Foo) {
    a.method();
}

fn main() {
    let x: u8 = 5u8;
    let y: String = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!");
}


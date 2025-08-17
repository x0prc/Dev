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
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

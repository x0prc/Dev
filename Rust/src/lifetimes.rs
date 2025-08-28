// sol : lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x: &str = "long";
    let y: &str = "longer";
    
    println!("{}", longest(x, y));
}

fn invalid_output<'a>(s: &'a str) -> &'a str {
    s
}

fn main() {
    let s: String = String::from("foo");
    
    let x = invalid_output(&s);
    
    println!("{}", x);
}

#[derive(Debug)]
struct Borrowed<`a>(&'a i32);

// Similarly, both references here must outlive this structure.
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    let x: i32 = 18;
    let y: i32 = 15;

    let single: Borrowed = Borrowed(&x);
    let double: NamedBorrowed = NamedBorrowed { x: &x, y: &y };
    let reference: Either = Either::Ref(&x);
    let number: Either    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}

fn main()
{ 
  /* 'a tied to fn-main stackframe */
  let var_a: u32 = 35;
  let example: Example;
  
  /* Lifetime 'b tied to new stackframe/scope */ 
    let var_b: NoCopyType = NoCopyType {};
    
    /* fixme */
    example = Example { a: &var_a, b: &var_b };
  
  println!("(Success!) {:?}", example);
}

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
#[allow(dead_code)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}

/* Fix function signature */
fn fix_me<'a>(foo: &'a Example) -> &'a NoCopyType
{ foo.b }

fn main()
{
    let no_copy: NoCopyType = NoCopyType {};
    let example: Example = Example { a: &1, b: &no_copy };
    fix_me(&example);
    println!("Success!")
}

// static lifetime lives throughout the program
struct ImportantExcerpt {
    part: &'static str,
}

impl ImportantExcerpt {
    fn level(& self) -> i32 {
        3
    }
}

fn main() {}


// elision 

fn input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

fn pass<'a>(x: &'a i32) -> &'a i32 { x }

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

struct Owner(i32);

impl Owner {
    // Annotate lifetimes as in a standalone function.
    fn add_one(&mut self) { self.0 += 1; }
    fn print(& self) {
        println!("`print`: {}", self.0);
    }
}

struct Person<'a> {
    age: u8,
    name: &'a str,
}

enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {}


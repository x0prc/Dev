fn main() {
    let coord = (2,3);
    println!("Coordinate: {:?}, {:?}", coord.0, coord.1); //tuple

    let(x, y) = (2,3);
    println!("x: {}, y: {}", x, y);
}

//expressions
enum Access {
    Read,
    Write,
    Execute
}

fn main(){
    let acess_level = Access::Read;
    let can_access_file = match acess_level {
        Access::Read => true,
        _ => false
    };
    println!("Can access file: {}", can_access_file);
}

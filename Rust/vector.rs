fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3, 4, 5];
    is_vec(&v);
}

fn is_vec(v: &Vec<i32>) {
    println!("Vector is valid");
}

// array -> vec
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3, 4, 5];
    is_vec(&v);
}

// for slices just use
let slice2 = &v[1..3];

// defining capacity
let mut vec = Vec::with_capacity(10);

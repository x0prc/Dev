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

// indexing

fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v.get(i))
    }

    for i in 0..5 {
        match v.get(i) {
            Some(e) => v[i] = e + 1,
            None => v.push(i + 2)
        }
    }
    
    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!");
}

// slicing
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    let slice1: Vec<i32> = &v[..];
    // Out of bounds will cause a panic
    // You must use `v.len` here
    let slice2 &[i32] = &v[0..v.len()];
    
    assert_eq!(slice1, slice2);
    
    // A slice can also be mutable, in which
    // case mutating it will mutate its underlying Vec.
    // Note: slice and &Vec are different
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3: &[i32] = &v[0..];

    assert_eq!(slice3, &[1, 2, 3, 42]);
    assert_eq!(v, &[1, 2, 3, 42]);

    println!("Success!");
}

// create capacity
fn main() {
    let mut vec = Vec::with_capacity(10);

    // The vector contains no items, even though it has capacity for more
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    // These are all done without reallocating...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    // ...but this may make the vector reallocate
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);


    // Fill in an appropriate value to make the `for` done without reallocating 
    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);
    
    println!("Success!");
} 
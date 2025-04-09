use std::collections::HashMap;

fn main(){
    let mut scores = HashMap::new();
    scores.insert("KKR", 100);
    scores.insert("RCB", 90);
    scores.insert("MI", 80);


    let score = scores.get("KKR");
    assert_eq!(score, Some(&100));
}
// capacity
let mut map: HashMap<i32, i32> = HashMap::with_capacity(10);
map.insert(1, 10);
map.insert(2, 20);
assert!(map.capacity() >= 10);

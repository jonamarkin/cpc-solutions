use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    println!("Hello, world!");
    let array = [6, 5, 6, 7];

    let aa = vec![1, 3, -1, -3, 5, 3, 6, 7];

    //println!("{}", sliding_window_maximum_brute_force(aa, 3));
    println!("{:?}", towers_hashmap_approach(&array));
}

fn towers_hashmap_approach(bars: &[i32]) -> (i32, i32) {
    let mut map = HashMap::new();
    for bar in bars {
        *map.entry(bar).or_insert(0) += 1;
    }
    let num_towers = map.values().count();
    let max_height = map.values().max().unwrap();

    (*max_height, num_towers as i32)
}

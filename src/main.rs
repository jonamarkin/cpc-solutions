use std::collections::HashMap;
use std::io;

fn main() {
    longest_k_good_segment();
}

pub fn longest_k_good_segment() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n_k: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = n_k[0];
    let k = n_k[1];

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut left = 0;
    let mut right = 0;
    let mut window = HashMap::new();
    let mut max_length = 0;
    let mut max_left = 0;
    let mut max_right = 0;

    while right < n {
        window.entry(a[right]).and_modify(|e| *e += 1).or_insert(1);
        while window.len() > k {
            let count = window.get_mut(&a[left]).unwrap();
            if *count == 1 {
                window.remove(&a[left]);
            } else {
                *count -= 1;
            }
            left += 1;
        }

        let length = right - left + 1;
        if length > max_length {
            max_length = length;
            max_left = left;
            max_right = right;
        }
        right += 1;
    }

    println!("{} {}", max_left + 1, max_right + 1);
    (max_left + 1, max_right + 1)
}

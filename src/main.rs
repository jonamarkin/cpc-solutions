use std::collections::{BTreeMap, BinaryHeap, HashSet, VecDeque};

fn main() {
    println!("Hello, world!");
    let array = [1, 3, -1, -3, 5, 3, 6, 7];

    let aa = vec![1, 3, -1, -3, 5, 3, 6, 7];

    //println!("{}", sliding_window_maximum_brute_force(aa, 3));
    println!("{:?}", max_sliding_window_bst(&array, 3));
}

fn max_sliding_window_bst(nums: &[i32], k: usize) -> Vec<i32> {
    // Edge cases check: if length of nums is 0 or k is 0 or 1 return empty vec
    let n = nums.len();
    if n * k == 0 {
        return vec![];
    }
    if k == 1 {
        return nums.to_vec();
    }

    // Initialize a BTreeMap to keep track of the elements in the sliding window
    let mut bst = BTreeMap::new();
    // Initialize an empty output vector
    let mut output = vec![];
    // Iterate through the elements of the nums vector
    for i in 0..n {
        // add current element to the bst by incrementing its count
        *bst.entry(nums[i]).or_insert(0) += 1;
        // if bst size is greater than k, remove the element at i-k
        if i >= k {
            if *bst.get(&nums[i - k]).unwrap() == 1 {
                bst.remove(&nums[i - k]);
            } else {
                *bst.get_mut(&nums[i - k]).unwrap() -= 1;
            }
        }
        // if the bst size is equal to k, add the max element to the output
        if i >= k - 1 {
            output.push(*bst.iter().rev().next().unwrap().0);
        }
    }
    output
}

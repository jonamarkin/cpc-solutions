use std::collections::{HashSet, VecDeque};

fn main() {
    println!("Hello, world!");
    let array = [1, 3, -1, -3, 5, 3, 6, 7];

    let aa = vec![1, 3, -1, -3, 5, 3, 6, 7];

    //println!("{}", sliding_window_maximum_brute_force(aa, 3));
    println!("{:?}", max_sliding_window(&array, 3));
}

pub fn max_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
    // Edge cases check: if length of nums is 0 or k is 0 or 1 return empty vec
    let n = nums.len();
    if n * k == 0 {
        return vec![];
    }
    if k == 1 {
        return nums.to_vec();
    }

    // Initialize deque with first k elements indexes
    let mut deq: Vec<usize> = (0..k).collect();
    // Find the index of the max element in the first k elements
    let mut max_idx = (0..k).max_by(|&i, &j| nums[i].cmp(&nums[j])).unwrap();
    // Initialize the output with the max element in the first k elements
    let mut output = vec![nums[max_idx]];

    // Iterate through the nums vector starting from the k-th element
    for i in k..n {
        //Remove the first element from the deque if it's not in the current sliding window
        if deq[0] == i - k {
            deq.remove(0);
        }
        //Remove elements from deque that are smaller than current element
        while !deq.is_empty() && nums[i] > nums[*deq.last().unwrap()] {
            deq.pop();
        }
        // Add current element to the deque
        deq.push(i);
        // Add the max element of the current sliding window to the output
        output.push(nums[deq[0]]);
    }
    output
}

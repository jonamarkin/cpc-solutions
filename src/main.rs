use std::env::current_exe;

fn main() {
    println!("Hello, world!");
    let array = [-1, 5, 8, -9, 4, 1];

    let max_sub = kandane_maxsubarray(&array);
    println!("The max is: {}", max_sub);
}

/**
 * max_subarray_bruteforce
 * Brute force approach
 */
fn max_subarray_bruteforce(array: &[i32]) -> i32 {
    let mut max_subarray = i32::MIN;

    for (i, el) in array.iter().enumerate() {
        let mut current_subarray = 0;

        for j in i..array.len() {
            current_subarray += array[j];
            max_subarray = std::cmp::max(max_subarray, current_subarray);
        }
    }
    return max_subarray;
}

/**
 * Kandane's Algorithm
 */
fn kandane_maxsubarray(nums: &[i32]) -> i32 {
    let mut current_subarray = 0;
    let mut max_subarray = i32::MIN;

    for i in nums.iter() {
        current_subarray = std::cmp::max(*i, current_subarray + i);
        max_subarray = std::cmp::max(current_subarray, max_subarray);
    }

    max_subarray
}

use std::collections::{BTreeMap, BinaryHeap, HashSet, VecDeque};

fn main() {
    println!("Hello, world!");
    let array = [8, 3, -1, -3, 5, 3, 6, 7];

    let aa = vec![1, 3, -1, -3, 5, 3, 6, 7];

    //println!("{}", sliding_window_maximum_brute_force(aa, 3));
    println!("{:?}", next_greater_elements(&array));
}

fn next_greater_elements(nums: &[i32]) -> Vec<i32> {
    let mut res = vec![-1; nums.len()];
    let mut stack = VecDeque::new();

    // Iterate through the input vector in reverse
    for i in (0..nums.len() * 2).rev() {
        // Iterate through the stack while the top of the stack has a smaller value than the current element
        while let Some(top) = stack.back() {
            if nums[*top] <= nums[i % nums.len()] {
                stack.pop_back();
            } else {
                break;
            }
        }

        // If the stack is not empty, the top of the stack is the next greater element
        if let Some(top) = stack.back() {
            res[i % nums.len()] = nums[*top];
        }
        // Push the current index onto the stack
        stack.push_back(i % nums.len());
    }
    res
}

use std::{
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque},
    io,
};

fn main() {
    println!("Hello, world!");
    // let array = [1, 0, 2];

    // let aa = vec![1, 20, 6, 4, 5];

    //println!("{}", sliding_window_maximum_brute_force(aa, 3));
    //println!("{:?}", towers_hashmap_approach(&array));

    //println!("{}", is_ideal_permutation(&array))

    let mut nums = vec![1, 2, 0];
    let result = ideal_permutations_merge_sort(&mut nums);
    println!("Is ideal permutation: {:?}", result);

    // let mut arr = vec![1, 20, 6, 4, 5];
    // let end = arr.len() - 1;

    // println!("{}", merge_sort_and_count(&mut arr, 0, end));
}

pub fn merge_and_count(nums: &mut Vec<i32>, left: usize, middle: usize, right: usize) -> i32 {
    // Left subarray
    let leftarray: Vec<i32> = nums[left..=middle].to_vec();

    // Right subarray
    let rightarray: Vec<i32> = nums[(middle + 1)..=right].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = left;
    let mut swaps = 0;

    while i < leftarray.len() && j < rightarray.len() {
        if leftarray[i] <= rightarray[j] {
            nums[k] = leftarray[i];
            i += 1;
        } else {
            nums[k] = rightarray[j];
            j += 1;
            swaps += (middle + 1) - (left + i);
        }
        k += 1;
    }

    while i < leftarray.len() {
        nums[k] = leftarray[i];
        i += 1;
        k += 1;
    }

    while j < rightarray.len() {
        nums[k] = rightarray[j];
        j += 1;
        k += 1;
    }

    return swaps as i32;
}

pub fn merge_sort_and_count(nums: &mut Vec<i32>, left: usize, right: usize) -> i32 {
    // Keeps track of the inversion count at a particular node of the recursion tree
    let mut count = 0;

    if (left < right) {
        let middle = (left + right) / 2;

        // Total inversion count = left subarray count
        // + right subarray count + merge count

        // Left subarray count
        count += merge_sort_and_count(nums, left, middle);

        // Right subarray count
        count += merge_sort_and_count(nums, middle + 1, right);

        // Merge count
        count += merge_and_count(nums, left, middle, right);
    }

    //Perform linear scan for local inversions

    return count;
}

pub fn ideal_permutations_merge_sort(nums: &mut Vec<i32>) -> bool {
    let mut local_inversions = 0;
    let end = nums.len() - 1;
    let global_inversions = merge_sort_and_count(nums, 0, end);
    for i in 0..nums.len() - 1 {
        if nums[i] > nums[i + 1] {
            local_inversions += 1;
        }
    }
    return local_inversions == global_inversions;
}

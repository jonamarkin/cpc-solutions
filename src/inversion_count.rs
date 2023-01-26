// You are given an integer array nums of length n which represents a permutation of all the integers in the range [0, n - 1].

// The number of global inversions is the number of the different pairs (i, j) where:

// 0 <= i < j < n
// nums[i] > nums[j]
// The number of local inversions is the number of indices i where:

// 0 <= i < n - 1
// nums[i] > nums[i + 1]
// Return true if the number of global inversions is equal to the number of local inversions.

//Brute force approach
//Loop twice
//All local inversions are global inversions as well
//So we want to find if there are any inversions that are not local
//Then that will make it false

pub fn is_ideal_permutation(A: &[i32]) -> bool {
    let N = A.len();
    for i in 0..N {
        for j in (i + 2)..N {
            if A[i] > A[j] {
                return false;
            }
        }
    }
    true
}

//Merge Sort Apporach
//Function to count number of inversios during the merge process
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

    return count;
}

// Time Complexity: O(n * log n),
// The algorithm used is divide and conquer i.e. merge sort whose complexity is O(n log n).
// Auxiliary Space: O(n), Temporary array.

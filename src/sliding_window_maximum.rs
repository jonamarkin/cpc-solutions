//Iterate over all the sliding windows and return the maximum for each
//There are  N-k+1 sliding windows
//There are k elements in the window
//Time complexity is 0(Nk)
pub fn sliding_window_maximum_brute_force(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let n = nums.len();

    //Handle if nums is empty or k is zero
    if n * k == 0 {
        return vec![0];
    }

    //Variable to hold the results and initialize with the number of windows
    let mut results: Vec<i32> = vec![0; n - k + 1];

    //Iterate over all the windows
    for i in 0..(n - k + 1) {
        //For every window
        //Initialize a variable to hold the max in this current window
        let mut max = i32::MIN;

        //Loop again to Compare with other elements in the window to get the max in te window
        for j in i..i + k {
            max = std::cmp::max(max, nums[j]);
        }

        results[i] = max;
    }

    return results;
}

//Using Deque to make the solution better
//This approach uses functional programming
//I pass a reference to the nums array in this case
// The time complexity of this approach is O(n), where n is the number of elements in the input nums vector.
// This is because the max_sliding_window_deque function iterates over each element of nums exactly once, performing a constant amount of work for each element.
// The space complexity of this approach is O(k), where k is the size of the sliding window.
// This is because the deq vector is used to store the indexes of the elements in the sliding window and its size is limited to k elements at any given time.
// The collect method's time complexity is O(n) where n is the number of elements in the iterator and space complexity is O(n) as it creates a new vector with n elements.
// The max_by method's time complexity is O(n) and space complexity is O(1). The remove(index) and push method's time complexity is O(n) and space complexity is O(1) as they insert/remove elements from vector.
// So overall the time and space complexity of this approach is O(n) and O(k) respectively which makes it quite efficient for large inputs.

pub fn max_sliding_window_deque(nums: &[i32], k: usize) -> Vec<i32> {
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

//Create an array that contains a concatenation of the provided array twice
//This way, we can loop in a circular way
//Uses time on O(n^2) and time of O(n) as well
pub fn next_greater_elements_brute_force(nums: &[i32]) -> Vec<i32> {
    // Create a vector to store the result with the same length as the input vector, filled with -1
    let mut res = vec![-1; nums.len()];
    // Create a new vector that is a concatenation of the input vector twice
    let mut doublenums = vec![0; nums.len() * 2];
    // Copy the input vector to the first half of the new vector
    doublenums[..nums.len()].copy_from_slice(nums);
    // Copy the input vector to the second half of the new vector
    doublenums[nums.len()..].copy_from_slice(nums);

    // Iterate through the input vector
    for i in 0..nums.len() {
        // Iterate through the new vector, starting from the next element of the current element
        for j in i + 1..doublenums.len() {
            // If the current element of the new vector is greater than the element from the input vector
            if doublenums[j] > doublenums[i] {
                // Assign the current element of the new vector to the corresponding element in the result vector
                res[i] = doublenums[j];
                // Break the inner loop
                break;
            }
        }
    }
    // Return the result vector
    res
}

//Stack Implementation

pub fn next_greater_elements_stack(nums: &[i32]) -> Vec<i32> {
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

// The modulo operator % is used in this code to calculate the remainder of dividing the current index i by the length of the input vector nums.len().
// This is used to ensure that the current index being processed is always within the range of the input vector, even when the iteration extends beyond the length of the input vector.

// When iterating through the input vector in reverse, the current index i will eventually reach a value less than nums.len() and the modulo operation will ensure that the index is "wrapped around" to the beginning of the input vector.
// This ensures that all elements of the input vector are considered in the loop.

// For example, if the input vector has a length of 3 and the current index is 5, the modulo operation will return the value 2 which corresponds to the second element of the input vector.

// It's also used to assign the value of next greater element to the corresponding index in the result array.

// It's used to ensure that the index doesn't exceed the length of the input array, and the result is always in the range of the input array.

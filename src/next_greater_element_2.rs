//Create an array that contains a concatenation of the provided array twice
//This way, we can loop in a circular way
//Uses time on O(n^2) and time of O(n) as well
fn next_greater_elements_brute_force(nums: &[i32]) -> Vec<i32> {
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

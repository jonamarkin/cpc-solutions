//Longest Increasing Subsequence
//Dynamic Programming Solution
//Can also be solved with backtracking but that will result in exponential time.
//DP approach will result in quadratic time which is better

// This function calculates the length of the longest increasing subsequence in an array of integers.
pub fn longest_increasing_subsequence(nums: &[i32]) -> i32 {
    // Initialize an array to store the maximum increasing subsequence ending at each index.
    // Fill it with the value 1, representing a single element as the minimum length of a subsequence.
    let mut dp = vec![1; nums.len()];

    // For each element in the array,
    for i in 1..nums.len() {
        // Compare it with every previous element,
        for j in 0..i {
            // If the current element is greater than the previous element,
            if nums[i] > nums[j] {
                // Update the maximum length of the increasing subsequence ending at the current index
                // by taking the maximum of the current value and the previous value plus one.
                dp[i] = std::cmp::max(dp[i], dp[j] + 1);
            }
        }
    }

    // Return the maximum value in the `dp` array, which represents the length of the longest increasing subsequence.
    *dp.iter().max().unwrap_or(&0)
}

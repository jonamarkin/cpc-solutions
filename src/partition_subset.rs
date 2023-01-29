use std::cmp::max;

//Dynamic  Programming
//An example of the 0-1 Knapsack Problem

fn can_partition(nums: &[i32]) -> bool {
    let sum: i32 = nums.iter().sum();
    if sum % 2 != 0 {
        return false;
    }
    let target = sum / 2;

    let n = nums.len();
    let mut dp = vec![false; (target + 1) as usize];
    dp[0] = true;

    for &num in nums {
        for i in (num..=target).rev() {
            dp[i as usize] = dp[i as usize] || dp[(i - num) as usize];
        }
    }

    return dp[target as usize];
}

fn main() {
    let nums = [1, 5, 11, 5];
    let result = can_partition(&nums);
    println!(
        "Can the array be partitioned into two subsets with equal sum: {}",
        result
    );
}

//Explanation of the Code
// The function can_partition takes a slice of integers (the array) as an argument and returns a boolean value indicating whether the array can be partitioned into two subsets with equal sum.
// We first calculate the sum of all elements in the array and check if it is an odd number.
// If it is, we return false as it is not possible to partition the array into two subsets with equal sum.
// If the sum is even, we calculate the target sum as sum / 2.
// We then create a boolean vector dp of size target + 1 to store the intermediate results.
// The dp[i] value represents whether it is possible to form a subset with sum i using elements from the array.
// The first element dp[0] is initialized to true, as it is always possible to form a subset with sum 0.
// The for loop over &num iterates through all the elements in the array.
// For each num, we iterate over i in the range (num..=target).rev().
// The .rev() method makes the loop iterate in reverse order, so we start from the largest possible sum and work our way down.
// At each iteration, we update dp[i as usize] as dp[i as usize] || dp[(i - num) as usize].
// This means that if it is possible to form a subset with sum i - num using elements from the array, it is also possible to form a subset with sum i by adding num to it.
// Finally, we return dp[target as usize], which indicates whether it is possible to form a subset with sum equal to target (which is half of the total sum of the array).

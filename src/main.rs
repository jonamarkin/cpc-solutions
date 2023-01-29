use std::cmp::max;

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
    println!("Can the array be partitioned into two subsets with equal sum: {}", result);
}

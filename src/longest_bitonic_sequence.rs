//Uses Dynamic Programming to Solve the Problem
//First find the Longest Increasing Subsequence(LIS) of the array
//Find the longest decreasing subsequence of the reverse array (LDS)
//Max length of the bitonic subsequence can be found by adding the length of the LIS and LDS and subtracting 1
//We subtract 1 because LIS and LDS will contain the common element of the sequence twice.

pub fn longest_bitonic_sequence(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut lis = vec![0; n];
    let mut lds = vec![0; n];

    // find the length of LIS for the given array
    for i in 0..n {
        lis[i] = 1;
        for j in 0..i {
            if nums[i] > nums[j] && lis[i] < lis[j] + 1 {
                lis[i] = lis[j] + 1;
            }
        }
    }

    // find the length of LDS for the reverse of the given array
    for i in (0..n).rev() {
        lds[i] = 1;
        for j in (i + 1..n).rev() {
            if nums[i] > nums[j] && lds[i] < lds[j] + 1 {
                lds[i] = lds[j] + 1;
            }
        }
    }

    // find the maximum length of Bitonic subsequence
    let mut maxLength = 0;
    for i in 0..n {
        maxLength = std::cmp::max(maxLength, lis[i] + lds[i] - 1);
    }
    return maxLength;
}

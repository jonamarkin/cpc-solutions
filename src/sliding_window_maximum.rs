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

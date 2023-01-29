use std::cmp::min;

// Function to calculate the minimum number of jumps
// to reach the end of the array
fn min_jumps(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut jumps = vec![std::i32::MAX; n]; // Vector to store minimum number of jumps

    jumps[0] = 0; // Initialize the minimum number of jumps to reach the first element as 0

    for i in 0..n {
        for j in 0..i {
            if (i <= j + arr[j] as usize) && (jumps[j] != std::i32::MAX) {
                jumps[i] = min(jumps[i], jumps[j] + 1);
                break;
            }
        }
    }

    // Return the minimum number of jumps to reach the end of the array
    return jumps[n - 1];
}

fn main() {
    let arr = [1, 3, 6, 1, 0, 9];
    let min_num_jumps = min_jumps(&arr);
    println!(
        "The minimum number of jumps to reach the end of the array is: {}",
        min_num_jumps
    );
}

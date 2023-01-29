use std::cmp::min;

//Dynamic Programming  problem
//Can also be solved with a recursive approach but this code uses the DP approach

//Explanation of approach
// The function min_jumps takes a slice of integers (the array) as an argument and returns an integer (the minimum number of jumps required to reach the end of the array).
// We first create a vector jumps to store the minimum number of jumps required to reach each element in the array.
// The first element of the array is initialized with 0, as it requires 0 jumps to reach the first element.
// The for loop over i iterates through all the elements in the array.
// For each i, the inner for loop over j checks if j + arr[j] is greater than or equal to i, and if jumps[j] is not equal to std::i32::MAX.
// If these conditions are met, the minimum number of jumps required to reach i is updated as min(jumps[i], jumps[j] + 1).
// The function finally returns jumps[n - 1], which represents the minimum number of jumps required to reach the end of the array.

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

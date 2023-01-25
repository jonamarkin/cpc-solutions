//this function takes rungs containing the height of each rung from the ground
//It also takes n, which is the number of rungs and tries to find the minimum strength factor
pub fn min_strength_factor(n: i32, rungs: Vec<i32>) -> i32 {
    // Create a new array b with size n and fill it with the difference of each rung with the previous one
    let mut b = vec![0; n as usize];
    b[0] = rungs[0];
    for i in 1..n {
        b[i as usize] = rungs[i as usize] - rungs[i as usize - 1];
    }

    // Sort the array b
    b.sort();

    // Find the largest value in array b and decrement it
    let mut x = b[n as usize - 1] - 1;
    // Create a variable to check whether all the other values in array b are smaller than the current value of x
    let mut tt = 0;
    for i in (0..n - 1).rev() {
        // Compare the current value of x with the current value of b
        if x == b[i as usize] {
            // If they are equal, decrement x
            x -= 1;
        } else if x < b[i as usize] {
            // If the current value of x is smaller than the current value of b, set tt to 1
            tt = 1;
            break;
        }
    }

    // Check if tt is 1, return the largest value in array b incremented by 1
    if tt == 1 {
        b[n as usize - 1] + 1
    } else {
        // Otherwise, return the largest value in array b
        b[n as usize - 1]
    }
}

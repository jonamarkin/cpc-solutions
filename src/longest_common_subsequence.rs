// Dynamic programming approach
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    // Create a 2D grid called "dp_grid" with `text1.len() + 1` rows and `text2.len() + 1` columns.
    let mut dp_grid = vec![vec![0; text2.len() + 1]; text1.len() + 1];

    // Loop through each column in "text2", starting from the last one.
    for col in (0..text2.len()).rev() {
        // Loop through each row in "text1", starting from the last one.
        for row in (0..text1.len()).rev() {
            // If the corresponding characters for this cell are the same...
            if text1.chars().nth(row) == text2.chars().nth(col) {
                // ...increment the length of the longest common subsequence by 1.
                dp_grid[row][col] = 1 + dp_grid[row + 1][col + 1];
            // Otherwise they must be different...
            } else {
                // ...so take the maximum of the lengths of the longest common subsequences
                // ending at the next characters in "text1" and "text2".
                dp_grid[row][col] = i32::max(dp_grid[row + 1][col], dp_grid[row][col + 1]);
            }
        }
    }

    // Return the value stored in "dp_grid[0][0]", which represents the length of the longest common subsequence.
    dp_grid[0][0]
}

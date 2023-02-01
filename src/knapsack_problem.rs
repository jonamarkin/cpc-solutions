//The 0/1 Knapsack Problem

//Recursion approach
pub fn knap_sack_recursion(W: i32, wt: &[i32], val: &[i32], n: i32) -> i32 {
    if n == 0 || W == 0 {
        return 0;
    }

    if wt[(n - 1) as usize] > W {
        return knap_sack_recursion(W, wt, val, n - 1);
    } else {
        return std::cmp::max(
            val[(n - 1) as usize] + knap_sack_recursion(W - wt[(n - 1) as usize], wt, val, n - 1),
            knap_sack_recursion(W, wt, val, n - 1),
        );
    }
}

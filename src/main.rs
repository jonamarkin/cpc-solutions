pub fn knap_sack(W: i32, wt: &[i32], val: &[i32], n: i32) -> i32 {
    if n == 0 || W == 0 {
        return 0;
    }

    if wt[(n - 1) as usize] > W {
        return knap_sack(W, wt, val, n - 1);
    } else {
        return std::cmp::max(
            val[(n - 1) as usize] + knap_sack(W - wt[(n - 1) as usize], wt, val, n - 1),
            knap_sack(W, wt, val, n - 1),
        );
    }
}
fn main() {
    let nums = [1, 5, 11, 5];
    // let result = longest_increasing_subsequence(&nums);
    // println!("Longest increasing subsequence: {}", result);
}

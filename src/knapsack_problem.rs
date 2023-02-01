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

//Memoization Approach
fn knap_sack_memo(W: i32, wt: &[i32], val: &[i32], n: i32, dp: &mut [[i32; 1001]; 1001]) -> i32 {
    if n == 0 || W == 0 {
        return 0;
    }

    if dp[n as usize][W as usize] != -1 {
        return dp[n as usize][W as usize];
    }

    if wt[(n - 1) as usize] > W {
        dp[n as usize][W as usize] = knap_sack_memo(W, wt, val, n - 1, dp);
        return dp[n as usize][W as usize];
    } else {
        dp[n as usize][W as usize] = std::cmp::max(
            (val[(n - 1) as usize] + knap_sack_memo(W - wt[(n - 1) as usize], wt, val, n - 1, dp)),
            knap_sack_memo(W, wt, val, n - 1, dp),
        );
        return dp[n as usize][W as usize];
    }
}
pub fn knap_sack_call(W: i32, wt: &[i32], val: &[i32], N: i32) -> i32 {
    let mut dp = [[-1; 1001]; 1001];
    knap_sack_memo(W, wt, val, N, &mut dp)
}

//Tabular Method
pub fn knap_sack_tabular(W: i32, wt: &[i32], val: &[i32], n: i32) -> i32 {
    let mut K = vec![vec![0; (W + 1) as usize]; (n + 1) as usize];

    for i in 0..=n {
        for w in 0..=W {
            if i == 0 || w == 0 {
                K[i as usize][w as usize] = 0;
            } else if wt[(i - 1) as usize] <= w {
                K[i as usize][w as usize] = std::cmp::max(
                    val[(i - 1) as usize]
                        + K[(i - 1) as usize][(w - wt[(i - 1) as usize]) as usize],
                    K[(i - 1) as usize][w as usize],
                );
            } else {
                K[i as usize][w as usize] = K[(i - 1) as usize][w as usize];
            }
        }
    }

    K[n as usize][W as usize]
}

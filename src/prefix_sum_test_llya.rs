fn main() {
    // input string
    let s = "......";
    let n = s.len();
    // number of queries
    let m = 4;
    // queries
    let queries = vec![(3, 4), (2, 3), (1, 6), (2, 6)];

    // create prefix sum array
    let mut prefix_sum = vec![0; n];
    for i in 0..n - 1 {
        prefix_sum[i + 1] = prefix_sum[i]
            + if s.chars().nth(i) == Some('.') && s.chars().nth(i + 1) == Some('.') {
                1
            } else {
                0
            };
    }

    // answer each query
    for i in 0..m {
        let (li, ri) = queries[i];
        let result = prefix_sum[ri - 1] - prefix_sum[li - 1];
        println!("{}", result);
    }
}

//Prefix Sum Approach
//Rationale

// The code reads the input from standard input, the first line is read as a string and the second line as an integer and the following lines as pairs of integers representing the queries.
// It then creates a prefix sum array, where each element represents the number of ".." or "##" from the start of the string till that index.
// After that for each query we simply calculate the difference between the prefix sum of the right and left end of the query.
// And print the answer for each query.
// The output is printed to the standard output, with one integer per line, representing the answers to the queries in the order in which they are given in the input.

use std::io;

fn main() {
    // input string
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap(); // read the first line of input
    s = s.trim().to_string(); // remove leading/trailing whitespaces
    let n = s.len(); // get the length of the string
                     // number of queries
    let mut m = String::new();
    io::stdin().read_line(&mut m).unwrap(); // read the second line of input
    let m: i32 = m.trim().parse().unwrap(); // parse the number of queries as i32
                                            // queries
    let mut queries = vec![];
    for _ in 0..m {
        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap(); // read the next line of input
        let query: Vec<i32> = query
            .trim()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect(); // split the line by space and parse the integers
        let (li, ri) = (query[0], query[1]); // unpack the integers
        queries.push((li, ri));
    }
    // create prefix sum array
    let mut prefix_sum = vec![0; n];
    for i in 0..n - 1 {
        prefix_sum[i + 1] = prefix_sum[i]
            + if (s.chars().nth(i) == Some('.') && s.chars().nth(i + 1) == Some('.'))
                || (s.chars().nth(i) == Some('#') && s.chars().nth(i + 1) == Some('#'))
            {
                1
            } else {
                0
            };
    }
    // answer each query
    for i in 0..m {
        let (li, ri) = queries[i as usize];
        let result = prefix_sum[(ri - 1) as usize] - prefix_sum[(li - 1) as usize];
        println!("{}", result);
    }
}

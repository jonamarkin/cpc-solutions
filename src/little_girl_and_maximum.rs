//"Little Girl and Maximum Sum" problem.
//Rationale
//Prefix Sum Approach
//Question Link - https://codeforces.com/problemset/problem/276/C?locale=en
// The task is to find the maximum sum of query replies after the array elements are reordered.

// The code first reads in the first line of input, which contains the number of elements in the array (n) and the number of queries (q).
// It then reads in the next line of input, which contains the elements of the array.

// The array is then sorted in ascending order.
// A new array, "val", is created and initialized to 0.
// For each query, the value at the index "l-1" in the "val" array is incremented by 1 and the value at the index "r" in the "val" array is decremented by 1 (if r is less than n).
// This is used to keep track of the number of times each element in the array is queried.

// The "val" array is then sorted in ascending order. A variable, "res", is initialized to 0.
// For each element in the array, the value at that index in the "val" array is multiplied by the corresponding element in the original array and added to "res".
// This gives the maximum sum of query replies after the array elements are reordered.

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n_q: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let n = n_q[0];
    let q = n_q[1];

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut a: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut c = vec![0; n as usize + 1];
    for _ in 0..q {
        input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let l_r: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let l = l_r[0] as usize;
        let r = l_r[1] as usize;
        c[l - 1] += 1;
        c[r] -= 1;
    }

    for i in 1..n {
        c[i as usize] += c[i as usize - 1];
    }

    a.sort();
    c.sort();

    let mut answer: i64 = 0;
    for i in 0..n {
        answer += (a[i as usize] as i64 * c[i as usize] as i64);
    }

    println!("{}", answer);
}

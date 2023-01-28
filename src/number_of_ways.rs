//Number of Ways
//Question Link: https://codeforces.com/problemset/problem/466/C?locale=en
//Rationale behind the code
// It starts by reading the input, which is the number of elements in the array and the elements themselves.
// Then it checks if the sum of the elements is divisible by 3, if not it prints 0 and exits.

// It then initializes two variables, suf and dp, and starts a loop that iterates through the array in reverse.
// In each iteration, it updates the suf variable with the current element of the array, and updates the dp array with the previous element if the current index is not the last element.
// If the suf is equal to one-third of the total sum, it increments the current element of the dp array.

// Then it initializes an ans variable to zero and starts another loop that iterates through the array in the forward direction.
// In each iteration, it updates the pre variable with the current element of the array, and if the pre is equal to one-third of the total sum, it increments the ans variable with the corresponding element of the dp array.
// Finally, it prints the ans variable as the output.

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: i32 = iter.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut sum = 0;
    for _ in 0..n {
        let x: i32 = iter.next().unwrap().parse().unwrap();
        a.push(x);
        sum += x;
    }

    if sum % 3 != 0 {
        println!("0");
        return;
    }

    let sum = sum / 3;
    let mut dp = vec![0; n as usize];
    let mut suf = 0;
    for i in (0..n).rev() {
        suf += a[i as usize];
        if i < n - 1 {
            dp[i as usize] = dp[i as usize + 1];
        }
        if suf == sum {
            dp[i as usize] += 1;
        }
    }

    let mut ans = 0;
    let mut pre = 0;
    for i in 0..n - 2 {
        pre += a[i as usize];
        if pre == sum {
            ans += dp[i as usize + 2];
        }
    }

    println!("{}", ans);
}

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

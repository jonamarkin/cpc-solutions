use std::{io, num};

fn number_of_ways(list: &[i64]) -> u64 {
    let mut s: i64 = list.iter().fold(0i64, |a, b| a + (*b as i64));

    if s % 3 != 0 {
        return 0;
    }

    s /= 3;

    let mut cnt = vec![0; list.len()];
    let mut ss: i64 = 0;
    for i in (0..=list.len() - 1).rev() {
        ss += list[i] as i64;
        if ss == s {
            cnt[i] = 1;
        }
    }
    for i in (0..=list.len() - 2).rev() {
        cnt[i] += cnt[i + 1];
    }

    let mut result = 0;
    ss = 0;
    for i in 0..list.len() - 2 {
        ss += list[i] as i64;
        if ss == s {
            result += cnt[i + 2];
        }
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", number_of_ways(&a));
}

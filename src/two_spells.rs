use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    io::{self, stdin, BufRead, Read},
};

fn main() {
    let mut n: usize = 0;
    let mut s_double = std::collections::BTreeSet::new();
    let mut sum = [0, 0];
    let mut s = [
        std::collections::BTreeSet::new(),
        std::collections::BTreeSet::new(),
    ];
    let mut cnt_double = [0, 0];

    let mut sc = std::io::stdin();
    let mut buffer = String::new();
    sc.read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();

    for _ in 0..n {
        buffer.clear();
        sc.read_line(&mut buffer).unwrap();
        let mut tokens = buffer.trim().split_whitespace();
        let tp: i32 = tokens.next().unwrap().parse().unwrap();
        let x: i32 = tokens.next().unwrap().parse().unwrap();

        if x > 0 {
            sum[0] += x;
            s[0].insert(x);
            cnt_double[0] += tp;
            if tp == 1 {
                s_double.insert(x);
            }
        } else {
            let x = -x;
            let id = if s[1].contains(&x) {
                1
            } else {
                assert!(s[0].contains(&x));
                0
            };

            sum[id] -= x;
            s[id].remove(&x);
            cnt_double[id] -= tp;
            if tp == 1 {
                assert!(s_double.contains(&x));
                s_double.remove(&x);
            }
        }

        let sum_double = cnt_double[0] + cnt_double[1];
        while s[1].len() < sum_double.try_into().unwrap() {
            upd(0, &mut sum, &mut s, &mut s_double, &mut cnt_double);
        }
        while s[1].len() > sum_double.try_into().unwrap() {
            upd(1, &mut sum, &mut s, &mut s_double, &mut cnt_double);
        }
        while s[1].len() > 0
            && s[0].len() > 0
            && *s[0].iter().next_back().unwrap() > *s[1].iter().next().unwrap()
        {
            upd(0, &mut sum, &mut s, &mut s_double, &mut cnt_double);
            upd(1, &mut sum, &mut s, &mut s_double, &mut cnt_double);
        }
        assert!(s[1].len() == sum_double.try_into().unwrap());

        let mut res = sum[0] + sum[1] * 2;
        if cnt_double[1] == sum_double && sum_double > 0 {
            res -= *s[1].iter().next().unwrap();
            if s[0].len() > 0 {
                res += *s[0].iter().next_back().unwrap();
            }
        }
        println!("{}", res);
    }
}

fn upd(
    id: usize,
    sum: &mut [i32; 2],
    s: &mut [BTreeSet<i32>; 2],
    s_double: &mut BTreeSet<i32>,
    cnt_double: &mut [i32; 2],
) {
    assert!(s[id].len() > 0);
    let x = *s[id].iter().next_back().unwrap();
    let x = if id == 1 {
        *s[id].iter().next().unwrap()
    } else {
        x
    };
    let d = s_double.contains(&x);

    sum[id] -= x;
    sum[(id + 1) % 2] += x;
    s[id].remove(&x);
    s[(id + 1) % 2].insert(x);
    cnt_double[id] -= if d { 1 } else { 0 };
    cnt_double[(id + 1) % 2] += if d { 1 } else { 0 };
}

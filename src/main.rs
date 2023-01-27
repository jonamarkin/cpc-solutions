use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    io::{self, stdin, BufRead, Read},
};

fn main() {
    //println!("Hello, world!");
    // let array = [1, 0, 2];

    // let aa = vec![1, 20, 6, 4, 5];

    //println!("{}", sliding_window_maximum_brute_force(aa, 3));
    //println!("{:?}", towers_hashmap_approach(&array));

    //println!("{}", is_ideal_permutation(&array))

    // let mut nums = vec![1, 2, 0];
    // let result = ideal_permutations_merge_sort(&mut nums);
    // println!("Is ideal permutation: {:?}", result);

    // let mut arr = vec![1, 20, 6, 4, 5];
    // let end = arr.len() - 1;

    //Maximum limit

    //const N: usize = (1e5 + 9) as usize;

    let mut n: usize = 0;
    let mut s_double: BTreeSet<i64> = std::collections::BTreeSet::new();
    let mut sum: [i64; 2] = [0, 0];
    let mut s: [BTreeSet<i64>; 2] = [
        std::collections::BTreeSet::new(),
        std::collections::BTreeSet::new(),
    ];
    let mut cnt_double: [i64; 2] = [0, 0];

    // 0: 0 -> 1
    // 1: 1 -> 0

    let mut sc = std::io::stdin();
    let mut buffer = String::new();
    sc.read_line(&mut buffer).unwrap();
    n = buffer.trim().parse().unwrap();

    for _ in 0..n {
        buffer.clear();
        sc.read_line(&mut buffer).unwrap();
        let mut tokens = buffer.trim().split_whitespace();
        let tp: i64 = tokens.next().unwrap().parse().unwrap();
        let x: i64 = tokens.next().unwrap().parse().unwrap();

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
        while s[1].len() < sum_double as usize {
            upd(0, &mut sum, &mut s, &mut s_double, &mut cnt_double);
        }
        while s[1].len() > sum_double as usize {
            upd(1, &mut sum, &mut s, &mut s_double, &mut cnt_double);
        }
        while s[1].len() > 0
            && s[0].len() > 0
            && *s[0].iter().next_back().unwrap() > *s[1].iter().next().unwrap()
        {
            upd(0, &mut sum, &mut s, &mut s_double, &mut cnt_double);
            upd(1, &mut sum, &mut s, &mut s_double, &mut cnt_double);
        }
        assert!(s[1].len() == sum_double as usize);

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
    sum: &mut [i64; 2],
    s: &mut [BTreeSet<i64>; 2],
    s_double: &mut BTreeSet<i64>,
    cnt_double: &mut [i64; 2],
) {
    assert!(s[id].len() > 0);
    let mut x = *s[id].iter().next_back().unwrap();
    x = if id == 1 {
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

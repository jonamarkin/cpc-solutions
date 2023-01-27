use std::collections::BTreeMap;
use std::io;

fn main() {
    let (n, m) = {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    let mut bt = BTreeMap::new();
    for i in 0..n {
        let (x, t) = {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        };
        bt.insert(x, (i, t, 0));
    }

    for _ in 0..m {
        let (p, b) = {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        };
        if let Some(x) = bt.range(..=p).next_back() {
            let (i, t, c) = x.1;
            let x22 = bt.iter().take(p);
            //bt.remove(x.0);
            bt.insert(*x.0, (*i, t + b, c + 1));
        }
        // if let Some(x) = bt.get_mut(&p) {
        //     let (i, t, c) = x;
        //     *x = (*i, *t + b, *c + 1);
        // }
        // if let Some(x) = bt.get_mut(&p) {
        //     let (i, t, c) = x;
        //     bt.remove(&p);
        //     bt.insert(p, (*i, *t + b, *c + 1));
        // }

        // if let Some(x) = bt.take(&p) {
        //     let (i, t, c) = x;
        //     bt.insert(p, (i, t + b, c + 1));
        // }
    }

    for x in bt.into_iter() {
        let (i, t, c) = x.1;
        println!("{} {}", c, t);
    }
}

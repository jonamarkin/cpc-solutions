//BST Implementation
//Does not provide the right answer
//Rationale Behind This Approach
// A BST is a type of tree data structure that allows efficient searching and insertion of elements, and it can be used to store the frogs in the order of their position.
// To implement this solution, we can first create a BST with the frogs, where the key of each node is the xi value of the frog (the position of the frog) and the value of each node is a tuple containing the initial length of the tongue of the frog and a counter for the number of mosquitoes eaten.
// For each mosquito, we can start at the root of the BST and move down the tree by comparing the position of the mosquito with the key of each node.
// When we reach a node where the key is greater than the position of the mosquito, we move left.
// If the key is less than the position of the mosquito, we move right.
// This way, we can find the leftmost frog that can eat the mosquito (i.e., the frog with the smallest xi value that is within the range of the mosquito).
// Once we find this frog, we can update the length of its tongue by adding the size of the mosquito and increment the number of mosquitoes that the frog has eaten.
// We can repeat this process for each mosquito landing and update the BST accordingly.
// Finally, we can print the number of mosquitoes eaten by each frog and the length of each frog's tongue after all mosquitoes have landed.

// The time complexity of this approach would be O(n log n + m log n) where n is the number of frogs and m is the number of mosquitoes as it requires sorting the frog and it will also take O(log n) time to extract the minimum element from the BST each time, which is similar to the priority queue solution.

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

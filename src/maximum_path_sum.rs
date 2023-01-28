//This implementation seeks to find the maximum path sum between two leaf nodes
//Question for reference
//https://practice.geeksforgeeks.org/problems/maximum-path-sum/1

//Rationale behind this implementation
// This code defines a Node struct with an i32 data field and two optional child nodes, a left and a right.
// The max_path_sum function takes a reference to the root node of a binary tree and returns the maximum path sum between two leaf nodes.
// It uses a helper function find_max to find the maximum path sum.
// It also uses a mutable variable maxi to keep track of the maximum path sum between two leaf nodes seen so far.
// The find_max function takes a reference to the current node and a mutable reference to max.
// It first checks if the current node is a leaf node and returns the data if it is.
// Otherwise, it recursively calls find_max on the left and right children of the current node.
// It then updates the maxi variable if the current node is not a leaf node and its left and right children are not None.
// Finally, it returns the maximum path sum of the left and right subtrees + the current node's data.

struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }
}

//Function to calculate the maximum path sum

fn max_path_sum(root: &Option<Box<Node>>) -> i32 {
    let mut max = std::i32::MIN;
    let temp = find_max(root, &mut max);
    if root.as_ref().unwrap().left.is_none() || root.as_ref().unwrap().right.is_none() {
        return std::cmp::max(temp, max);
    }
    return max;
}

//helper function
fn find_max(node: &Option<Box<Node>>, maxi: &mut i32) -> i32 {
    if node.is_none() {
        return std::i32::MIN;
    }
    if node.as_ref().unwrap().left.is_none() && node.as_ref().unwrap().right.is_none() {
        return node.as_ref().unwrap().data;
    }
    let lh = std::cmp::max(std::i32::MIN, find_max(&node.as_ref().unwrap().left, maxi));
    let rh = std::cmp::max(std::i32::MIN, find_max(&node.as_ref().unwrap().right, maxi));
    if node.as_ref().unwrap().left.is_some() && node.as_ref().unwrap().right.is_some() {
        *maxi = std::cmp::max(*maxi, lh + rh + node.as_ref().unwrap().data);
    }
    return node.as_ref().unwrap().data + std::cmp::max(lh, rh);
}

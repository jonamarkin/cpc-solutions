//Sliding Window Approach
//The idea is to maintain a window of size k, where k is the maximum number of different values allowed in the segment.
// Initialize two pointers, left and right, to the first element of the array.
// Create a data structure (e.g. a hash set or a map) to keep track of the distinct values in the current window.
// Iterate through the array using the right pointer, adding each element to the data structure and expanding the window.
// As soon as the size of the data structure exceeds k, remove the element at the left pointer from the data structure and move the left pointer one step to the right.
// Keep track of the length of the current window and update the answer if a longer k-good segment is found.
// Repeat steps until the right pointer reaches the end of the array.
// When the right pointer reaches the end of the array, the longest k-good segment will be the one with the longest length that was found during the iteration.

// This solution will have a time complexity of O(n) and a space complexity of O(k).

pub fn longest_k_good_segment() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n_k: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = n_k[0];
    let k = n_k[1];

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut left = 0;
    let mut right = 0;
    let mut window = HashMap::new();
    let mut max_length = 0;
    let mut max_left = 0;
    let mut max_right = 0;

    while right < n {
        window.entry(a[right]).and_modify(|e| *e += 1).or_insert(1);
        while window.len() > k {
            let count = window.get_mut(&a[left]).unwrap();
            if *count == 1 {
                window.remove(&a[left]);
            } else {
                *count -= 1;
            }
            left += 1;
        }

        let length = right - left + 1;
        if length > max_length {
            max_length = length;
            max_left = left;
            max_right = right;
        }
        right += 1;
    }

    println!("{} {}", max_left + 1, max_right + 1);
    (max_left + 1, max_right + 1)
}

//TODO: Try the segment tree approach as well

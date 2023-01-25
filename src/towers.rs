//Towers
//Uses the hashmap to solve the problem
// This code uses a HashMap to keep track of the count of each unique bar length.
// Then it iterates through the input bars array and for each element, it uses the entry method on the HashMap to check if the bar length already exists in the map.
// If it does, it increments the count of that bar length by 1.
// If it doesn't exist, it creates a new entry in the map with the count of 1.
// Then it uses the values method on the HashMap to get an iterator of all the counts and use the count method on that iterator to get the total number of unique bar lengths. It also uses the max method to find the maximum count of a bar length, which represents the height of the tallest tower.
// Finally, it returns a tuple of the maximum height and total number of towers.

pub fn towers_hashmap_approach(bars: &[i32]) -> (i32, i32) {
    let mut map = HashMap::new();
    for bar in bars {
        *map.entry(bar).or_insert(0) += 1;
    }
    let num_towers = map.values().count();
    let max_height = map.values().max().unwrap();

    (*max_height, num_towers as i32)
}

// The time complexity of this code is O(n) where n is the number of bars.
// The first loop iterates through the input array of bars which has a length of n, so it takes O(n) time.
// The entry method and the or_insert method both have a time complexity of O(1) on average, which means that the overall time complexity of this code is O(n) because the loop iterates n times.
// The space complexity of this code is also O(n) because it uses a HashMap to store the count of each unique bar length, which can be at most n.
// In the worst case, where all the bars have different lengths, the space complexity would be O(n) which is the same as the time complexity.

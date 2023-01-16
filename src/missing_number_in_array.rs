pub fn missing_number_sort(nums: Vec<i32>) -> i32 {
    //First sort the array
    //Then compare the indexes and the elements for mismatch since array is from 0 to n
    //Return index of mismatch as missing number
    //If no mismatch is found, return n as the missing number

    let mut mutable_nums = nums;
    mutable_nums.sort();

    for i in 0..mutable_nums.len() {
        if mutable_nums[i] != i as i32 {
            return i as i32;
        }
    }

    return mutable_nums.len() as i32;

    //This implementation uses O(NlongN) time to sort and also use O(N) space
}

pub fn missing_number_hashset(nums: Vec<i32>) -> i32 {
    //Declare a hashset to hold the values
    //Insert the values of nums into the hashset
    //Loop from 0 to n and compare if the every value in the range exists in the hashset
    //Return the value not contained in the hashset

    let mut nums_hash = HashSet::new();
    for i in 0..nums.len() {
        nums_hash.insert(nums[i]);
    }

    for i in 0..nums.len() {
        if !nums_hash.contains(&i.try_into().unwrap()) {
            return i as i32;
        }
    }

    return nums.len() as i32;
}

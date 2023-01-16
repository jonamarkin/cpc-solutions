//Base Cases
//We cannot trap water with 1 block
//we cannot trap water with two blocks as well
//We need at least three blocks to form a pocket to trap water
//Blocks in ascending order of descending order cannot trap water
//Height of water level is min(max_left_boundary, max_right_boundary)
//Trapped water is water level minus current index
//Width is always 1 so you don't need to multiply to find the area
//so formula is min(max_left_boundary, max_right_boundary) - height of block
//Water can never be stored in the first or last block so start fro index 1
pub fn trap(height: Vec<i32>) -> i32 {
    let mut height = height;
    let size = height.len();
    let mut result = 0;

    //Check if the blocks can trap water
    if size <= 2 {
        return 0;
    }

    //Create arrays to store the maximum to the left of every index and
    //max to the right off every index too. So two arrays
    let mut leftmax: Vec<i32> = vec![0; size];
    let mut rightmax: Vec<i32> = vec![0; size];

    //fill up the leftmax array
    leftmax[0] = height[0];
    for i in 1..size {
        //Start loop from 1 because 0 will have nothing to the left
        //The left max of the current index is obtained by comparing the value of
        //the current index in the original array "height" to the leftmax value of the
        //previous position in the leftmax array since that's the current leftmax value'
        leftmax[i] = std::cmp::max(height[i], leftmax[i - 1]);
    }

    //Apply a similar principle for the rightmax array but in the oppositee direction
    rightmax[size - 1] = height[size - 1];
    // for i in height.iter().rev().take(size - 1) {
    //     rightmax[i] = std::cmp::max(
    //         height[i.try_into().unwrap()],
    //         rightmax[i.try_into().unwrap() + 1],
    //     );
    // }

    //Another way of writing the above block of code
    for i in (0..(size - 1)).rev() {
        rightmax[i] = std::cmp::max(height[i], rightmax[i + 1]);
    }

    //Now we find the max trapped water
    //We loop and apply the formular min(leftmax, rightmax) - currentindex to every index
    for i in 0..size {
        result += std::cmp::min(leftmax[i], rightmax[i]) - height[i];
    }

    return result;
}

fn main() {
    println!("Hello, world!");
    let array = [56, 24, 8, 45, 6, 3, 1];

    let leaders: Vec<i32> = get_leaders(&array);
    println!("{:?}", leaders);
}

fn get_leaders(array: &[i32]) -> Vec<i32> {
    //Initialize  variable to hold max
    let mut max = 0;

    //Initialize vector to hold leaders
    let mut v = Vec::with_capacity(array.len());

    //Iterate the array from the right to the left
    //Since a leader will have to be greater than all elements to its right
    //We don't want to traverse all the elements to the right of every element and compare
    for i in array.iter().rev() {
        //Check if current element is greater than max
        if i >= &max {
            //Set max to i

            max = *i;
            //Add i to leaders
            v.push(*i);
        }
    }
    return v;
}

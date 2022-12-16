fn main() {
    //Find leaders in an array
    let array = [56, 24, 8, 45, 6, 3, 1];

    get_leaders(&array);
}

fn get_leaders(array: &[i32]) -> Vec<T> {
    //Initialize  variable to hold max
    let mut max = 0;

    //Initialize vector to hold leaders
    let mut v = Vec::new();

    for (i, item) in array.iter().rev() {
        //Check if current element is greater than max
        if i > max {
            //Set max to i

            max = i;
            //Add i to leaders
            v.push(i);
        }
    }
    return v;
}

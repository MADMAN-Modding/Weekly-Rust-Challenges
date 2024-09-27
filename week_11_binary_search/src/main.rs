fn main() {
    let sorted_array:Vec<i32> = [1,2,40,50,100,124,214,523,552,36463,19412412].to_vec();    

    let location = search(sorted_array, 214);

    println!("{}", if location != -1 {format!("Found at {}", location)} else {"Target was not found".to_string()})
}

fn search(sorted_array:Vec<i32>, target:i32) -> i16 {
    let mut left:usize = 0;
    let mut right:usize = (sorted_array.len()-1).try_into().unwrap();

    while left <= right {
        let middle:usize = (left + right) / 2; 
        if sorted_array[middle] < target {
            left = middle + 1;
        } else if sorted_array[middle] > target {
            right = middle - 1;
        } else {
            return middle as i16;
        }
    }

    return -1;
}
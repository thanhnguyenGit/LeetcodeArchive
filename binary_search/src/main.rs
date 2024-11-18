fn main() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    search(nums, 9);
}

//problem 704. Binary search
fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left: isize = 0;
    let mut right: isize = (nums.len() - 1) as isize;
    println!("origin left - right: {} - {}", left, right);
    while left <= right {
        let mid_point = (right + left) / 2;
        println!("mid point {mid_point} - {}", nums[mid_point as usize]);
        match nums[mid_point as usize].cmp(&target) {
            std::cmp::Ordering::Greater => {
                right = mid_point - 1;
                println!("left {right} - {}", nums[right as usize]);
                println!("updated mid point {mid_point}")
            }
            std::cmp::Ordering::Less => {
                left = mid_point + 1;
                println!("right {left} - {}", nums[left as usize]);
                println!(
                    "updated mid point {mid_point} - {}",
                    nums[mid_point as usize]
                )
            }
            std::cmp::Ordering::Equal => {
                println!("{}", nums.get(mid_point as usize).unwrap());
                return mid_point as i32;
            }
        }
    }
    -1
}

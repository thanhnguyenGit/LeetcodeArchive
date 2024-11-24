use std::usize;

fn main() {
    // let nums = vec![vec![1, 3, 5, 6], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let nums = vec![vec![1]];
    search_matrix(nums, 0);
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

//problem 74. Search a 2D Matrix - Medium
fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut top_row: i32 = 0;
    let mut bot_row: i32 = matrix.len() as i32 - 1;
    let mut mid_row: i32 = 0;
    while top_row <= bot_row {
        mid_row = (top_row + bot_row) / 2;
        if matrix[mid_row as usize][0] > target {
            bot_row = mid_row - 1;
            println!("top_row");
            println!("bot_row")
        } else if matrix[mid_row as usize][matrix[0].len() - 1] < target {
            top_row = mid_row + 1;
            println!("top_row");
            println!("bot_row")
        } else {
            break;
        }
    }
    if top_row > bot_row {
        return false;
    }
    match matrix[mid_row as usize].binary_search(&target) {
        Ok(val) => return true,
        Err(_) => return false,
    }
}

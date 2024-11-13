use std::{
    collections::{HashMap, HashSet},
    usize, vec,
};

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    three_sum(nums);
}

//problem 125: Valid Panlindrome - Easy
fn is_palindrome(s: String) -> bool {
    println!("this is original String: {}", s);
    let mut res_vec = Vec::new();
    let mut res_string = Vec::new();
    for char in s.chars() {
        match char {
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                res_vec.push(char.to_lowercase().to_string());
            }
            _ => continue,
        }
    }
    println!(
        "this is remove any non-aplanumeric character: {:?}",
        res_vec
    );
    let mut clone_res_vec = res_vec.clone();
    while let Some(char) = clone_res_vec.pop() {
        res_string.push(char);
    }

    println!(
        "this is the temp string made from res_vec: {:?}",
        res_string
    );
    assert_ne!(res_string.len(), 0);
    assert_ne!(res_vec.len(), 0);
    res_vec == res_string
}

//problem 167: Two Sum II - Input Array Is Sorted
fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut tmp_hashmap = HashMap::with_capacity(numbers.capacity());
    for (index, value) in numbers.iter().enumerate() {
        match tmp_hashmap.get(&(target - *value)) {
            Some(&index1) => return vec![index1 + 1, (index + 1) as i32],
            None => tmp_hashmap.insert(*value, index as i32),
        };
    }

    vec![]
}

//problem 15: 3Sum
fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable(); //O(nlogn)
    println!("sorted input nums {:?}", nums);
    let mut result_set: HashSet<Vec<i32>> = HashSet::new();
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            // 2-nested loop: O(n^2)
            println!("candidates : {}:{} {}:{}", i, nums[i], j, nums[j]);
            let target = -nums[i] - nums[j];
            println!("Look for target {} in {:?}", target, &nums[j + 1..]);
            let result = match &nums[j + 1..].binary_search(&target) {
                // binary_search: O(logn)
                Ok(value) => {
                    let tmp_vec = &nums[j + 1..];
                    let vec = vec![nums[i], nums[j], tmp_vec[*value]];
                    println!(
                        "found {} {},create a new test_vec : {:?}",
                        value, tmp_vec[*value], vec
                    );
                    result_set.insert(vec);
                    Some(*value)
                }
                Err(_) => None,
            };
        }
    }
    //total (O(n^2 logn))
    println!("Final result set : {:?}", result_set);
    result_set.into_iter().collect::<Vec<Vec<i32>>>()
}

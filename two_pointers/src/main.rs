use std::{collections::HashMap, vec};

fn main() {
    let s = "0P".to_string();
    is_palindrome(s);
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
fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    vec![vec![]]
}

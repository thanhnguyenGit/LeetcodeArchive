use std::collections::{BTreeMap, HashMap};

fn main() {
    let vec = vec![2, 7, 11, 15];
    two_sum(vec, 9);
}

// Problem 210: Contain duplication - Easy
fn contain_duplication(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;
    let mut set_num = HashSet::new();
    for element in nums.iter() {
        set_num.insert(element);
    }
    println!("this is {:?}", set_num);
    assert_ne!(nums.len(), 0);
    assert_ne!(set_num.len(), 0);
    let nums_actor = vec![1, 2, 3, 1];
    //assert_eq!(contain_duplication(nums_actor), true);
    return set_num.len() < nums.len();
}

//problem 242: Valid Anagram - Easy
fn is_anagram(s: String, t: String) -> bool {
    let sorted_s = sort_string(s);
    let sorted_t = sort_string(t);
    assert_eq!(sorted_s, sorted_t);
    return sorted_s == sorted_t;
}

fn sort_string(input: String) -> String {
    let mut char_vec: Vec<char> = input.chars().collect();
    char_vec.sort_unstable();
    let output: String = char_vec.into_iter().collect();
    output
}

//problem 1: Two Sum - Easy
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut tmp_hashmap = HashMap::with_capacity(nums.capacity());
    for (index, value) in nums.iter().enumerate() {
        match tmp_hashmap.get(&(target - *value)) {
            Some(&index2) => return vec![index as i32, index2],
            None => tmp_hashmap.insert(*value, index as i32),
        };
    }
    vec![]
}

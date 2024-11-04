use std::{
    char,
    cmp::Reverse,
    collections::{BTreeMap, HashMap},
    iter,
    ops::Deref,
    usize, vec,
};

fn main() {
    let s = "3[a2[c]]".to_string();
    decode_string(s);
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

//problem 49: Group Anagrams - Medium
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result_map: HashMap<String, Vec<String>> = HashMap::new();
    for elements in strs.into_iter() {
        let mut sorted_elements_as_key = sort_string_ref(elements.deref());
        println!("elements after sort as key: {}", sorted_elements_as_key);
        result_map
            .entry(sorted_elements_as_key)
            .or_insert(Vec::new())
            .push(elements);
    }
    println!("{:?}", result_map);
    let result = result_map.into_values().collect();
    println!("{:?}", result);
    result
}
fn sort_string_ref(input: &str) -> String {
    let mut char_vec: Vec<char> = input.chars().collect();
    char_vec.sort_unstable();
    let output: String = char_vec.into_iter().collect();
    output
}

//problem 347. Top K Frequent Elements - Medium
fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut count_map: HashMap<i32, i32> = HashMap::new();
    for element in nums.into_iter() {
        *count_map.entry(element).or_default() += 1;
    }
    assert_ne!(count_map.len(), 0);
    println!("map {:?}", count_map);
    let mut sorted = count_map
        .iter()
        .map(|(&key, &value)| (key, value))
        .collect::<Vec<(i32, i32)>>();
    sorted.sort_by_key(|&(_, value)| Reverse(value));
    assert_ne!(sorted.len(), 0);
    println!("sorted {:?}", sorted);
    let res_vec: Vec<i32> = sorted
        .into_iter()
        .take(k as usize)
        .map(|(key, _)| key)
        .collect();
    println!("res_vec {:?}", res_vec);
    res_vec
}

//problem 394: Decode String - Medium
pub fn decode_string(s: String) -> String {
    let mut num = String::from("");
    let mut curr = String::from("");
    let mut stack = vec!["".to_string()];

    for char in s.chars() {
        match char {
            '0'..='9' => num.push(char),
            '[' => {
                stack.push(curr);
                stack.push(num);
                curr = "".to_string();
                num = "".to_string();
            }
            ']' => {
                let mut n = stack.pop().unwrap().parse::<usize>().unwrap();
                curr = curr.repeat(n);
                let mut prev = stack.pop().unwrap().to_string();
                curr = prev + &curr;
            }
            _ => {
                curr.push(char);
            }
        }
    }
    curr
}

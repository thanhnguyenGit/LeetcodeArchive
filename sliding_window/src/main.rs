use std::collections::{HashMap, HashSet};

fn main() {
    let s = "qrsvbspk".to_string();
    length_of_longest_substring(s);
}

//problem 121: Best time to buy and Sell stock - Easy
fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy = prices[0];
    let mut esstimate_profit = 0;
    for price in prices.iter() {
        buy = buy.min(*price);
        esstimate_profit = esstimate_profit.max(price - buy);
        println!("buy price at : {buy}, profit {esstimate_profit}")
    }
    return if prices.is_empty() {
        0
    } else {
        esstimate_profit
    };
}

//problem 3. Longest Substring without Repeating Characters
fn length_of_longest_substring(s: String) -> i32 {
    let mut set = HashSet::new();
    let mut vec: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut res = 0;
    for right in 0..vec.len() {
        while (set.contains(&vec[right])) {
            println!("pre set {set:?}");
            set.remove(&vec[left]);
            left += 1;
            println!("post set {set:?}");
        }
        set.insert(&vec[right]);
        res = res.max(set.len());
        println!("set {set:?}")
    }
    println!("res {res}");
    res as i32
}

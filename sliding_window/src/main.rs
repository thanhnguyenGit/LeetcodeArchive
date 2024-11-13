use std::collections::HashMap;

fn main() {
    let prices = vec![7, 1, 6, 5, 4, 2];
    max_profit(prices);
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

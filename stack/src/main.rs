use std::{collections::HashMap, i32::MAX};

fn main() {
    minstack_runner();
}

//problem 20: Valid Parentheses - Easy:
pub fn is_valid(s: String) -> bool {
    let mut map = HashMap::new();
    map.insert('(', ')');
    map.insert('[', ']');
    map.insert('{', '}');
    let mut stack: Vec<char> = Vec::new();

    for char in s.chars() {
        match char {
            '(' | '[' | '{' => {
                stack.push(char);
                println!("current stack: {:?}", stack)
            }
            _ => {
                println!("current char: '{}'", char);
                let candidate = map.get(stack.last().expect("Top of the stack"));
                println!(
                    "candidate '{}', current char '{}'",
                    *candidate.unwrap(),
                    char
                );
                if char == *candidate.unwrap() {
                    stack.pop();
                    println!("stack after pop: {:?}", stack);
                } else {
                    println!("THIS IS INCORRECT!!!!!!!!");
                    return false;
                };
            }
        }
    }
    if stack.len() != 0 {
        return false;
    }
    true
}

//problem 155: Min stack - Medium
//
type minsta_value = i32;
type minsta_smallest_value = i32;

struct MinStack(Vec<(minsta_value, minsta_smallest_value)>);

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, val: i32) {
        println!("pushing");
        if let Some((_, smallest)) = self.0.last() {
            self.0.push((val, std::cmp::min(val, *smallest)));
            println!("{:?}", self.0)
        } else {
            self.0.push((val, val));
        };
    }

    fn pop(&mut self) {
        self.0.pop().expect("Top value on the stack");
    }

    fn top(&self) -> i32 {
        self.0.last().expect("Last value").0
    }

    fn get_min(&self) -> i32 {
        self.0.last().expect("Get the smallest from top value").1
    }
}

fn minstack_runner() {
    let mut minsta = MinStack::new();
    minsta.push(12);
    minsta.push(20);
    minsta.push(10);
    minsta.push(27);

    minsta.top();
}

use std::collections::HashMap;

fn main() {
    generate_parenthesis(3);
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

//problem 150. Evalute Reverse Polish Notation
fn eval_rpn(tokens: Vec<String>) -> i32 {
    println!("original string: {tokens:?}");
    let mut stack = Vec::new();
    for element in tokens.iter() {
        match element.as_str() {
            "+" => {
                println!("found '+' operator");
                let val1 = stack.pop().expect("Top element");
                let val2 = stack.pop().expect("Below top element");
                let res = val2 + val1;
                println!("val1 : {val1}, val2 : {val2}, res : {res}");
                stack.push(res);
            }
            "-" => {
                println!("found '-' operator");
                let val1 = stack.pop().expect("Top element");
                let val2 = stack.pop().expect("Below top element");
                let res = val2 - val1;
                println!("val1 : {val1}, val2 : {val2}, res : {res}");
                stack.push(res);
            }
            "*" => {
                println!("found '*' operator");
                let val1 = stack.pop().expect("Top element");
                let val2 = stack.pop().expect("Below top element");
                let res = val2 * val1;
                println!("val1 : {val1}, val2 : {val2}, res : {res}");
                stack.push(res);
            }
            "/" => {
                println!("found '/' operator");
                let val1 = stack.pop().expect("Top element");
                let val2 = stack.pop().expect("Below top element");
                let res = val2 / val1;
                stack.push(res)
            }
            _ => {
                stack.push(element.parse::<i32>().expect("Turn to i32"));
            }
        }
    }
    println!("{:?}", stack);
    *stack.last().expect("Only one value")
}

//problem 22: Generate Parentheses - Medium
//
fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = Vec::new();
    let mut s = String::new();
    backtrack(&mut res, &mut s, 0, 0, n);
    res
}

fn backtrack(res: &mut Vec<String>, s: &mut String, open: i32, close: i32, n: i32) {
    if close == open && open == n {
        res.push(s.clone());
        println!("res : {res:?}");
        return;
    }

    if open < n {
        s.push('(');
        println!("s : {s}");
        backtrack(res, s, open + 1, close, n);
        s.pop();
        println!("s after backtrack : {s}")
    }

    if close < open {
        s.push(')');
        println!("s : {s}");
        backtrack(res, s, open, close + 1, n);
        s.pop();
        println!("s after backtrack : {s}")
    }
}

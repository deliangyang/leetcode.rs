use std::cmp::max;

pub fn longest_valid_parentheses(s: String) -> i32 {
    let l = s.len() as i32;
    if l == 1 {
        return 0
    }
    let mut stack: Vec<i32> = vec![];
    let chars = s.as_bytes();
    let mut c;
    let mut maxans = 0;
    stack.push(-1);
    for i in 0..l {
        c = chars[i as usize] as char;
        if c == '(' {
            stack.push(i)
        } else {
            stack.pop();
            if stack.is_empty() {
                stack.push(i)
            } else {
                maxans = max(maxans, i - *stack.last().unwrap())
            }
        }
    }
    maxans
}

fn main() {
    println!("{}", longest_valid_parentheses("(())()(()))".to_string()))
}
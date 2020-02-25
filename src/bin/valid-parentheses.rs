fn is_valid(s: String) -> bool {
    let l = s.len();
    if l == 0 {
        return true;
    } else if l % 2 == 1 {
        return false;
    }
    let mut stack = vec![];
    for c in s.chars() {
        if c == '{' || c == '[' || c == '(' {
            stack.push(c);
            continue;
        }
        if stack.is_empty() {
            return false;
        } else {
            let top = *stack.last().unwrap();
            if (top == '{' && c == '}') || (top == '[' && c == ']')
                || (top == '(' && c == ')') {
                stack.pop().unwrap();
            } else {
                return false;
            }
        }
    }
    stack.is_empty()
}

fn main() {
    assert_eq!(is_valid("()".to_string()), true);
    assert_eq!(is_valid("()[]{}".to_string()), true);
    assert_eq!(is_valid("(]".to_string()), false);
    assert_eq!(is_valid("([)]".to_string()), false);
    assert_eq!(is_valid("{[]}".to_string()), true);
    assert_eq!(is_valid("".to_string()), true);
}
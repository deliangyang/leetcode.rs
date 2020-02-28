fn generate_parenthesis(n: i32) -> Vec<String> {
    fn generate(n: i32) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        if n == 0 {
            res.push("".to_string());
        } else {
            for i in 0..n {
                for left in &generate(i) {
                    for right in &generate(n - 1 - i) {
                        let t = String::from("(".to_string()) + left
                            + String::from(")".to_string()).as_ref() + right;
                        res.push(t);
                    }
                }
            }
        }
        res
    }
    generate(n)
}

/**
 * https://leetcode-cn.com/problems/generate-parentheses/
**/
fn main() {
    let res = vec![
        "()()()".to_string(),
        "()(())".to_string(),
        "(())()".to_string(),
        "(()())".to_string(),
        "((()))".to_string(),
    ];
    assert_eq!(generate_parenthesis(3), res);
}
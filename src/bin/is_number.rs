use std::collections::HashMap;

fn is_number(s: String) -> bool {
    let finals = vec![0, 0, 0, 1, 0, 1, 1, 0, 1];
    let transfer: Vec<Vec<i32>> = vec![
        vec![0, 1, 6, 2, -1, -1],
        vec![-1, -1, 6, 2, -1, -1],
        vec![-1, -1, 3, -1, -1, -1],
        vec![8, -1, 3, -1, 4, -1],
        vec![-1, 7, 5, -1, -1, -1],
        vec![8, -1, 5, -1, -1, -1],
        vec![8, -1, 6, 3, 4, -1],
        vec![-1, -1, 5, -1, -1, -1],
        vec![8, -1, -1, -1, -1, -1]
    ];

    let mut state = 0;
    let make = |c| {
        match c {
            ' ' => 0,
            '+' | '-' => 1,
            '.' => 3,
            'e' | 'E' => 4,
            _ => {
                let code = c as i32;
                return if code >= 48 && code <=57 {
                    2
                } else {
                    5
                }
            }
        }
    };

    for c in s.chars() {
        state = transfer[state as usize][make(c) as usize];
        if state < 0 {
            return false;
        }
    }
    finals[state as usize] == 1
}

fn main() {
    let mut a = HashMap::new();
    a.insert("0", true);
    a.insert(" 0.1 ", true);
    a.insert("abc", false);
    a.insert("1 a", false);
    a.insert("2e10", true);
    a.insert(" -90e3    ", true);
    a.insert(" 1e", false);
    a.insert(" e3", false);
    a.insert(" 6e-1", true);
    a.insert(" 99e2.5 ", false);
    a.insert("53.5e93", true);
    a.insert(" --6", false);
    a.insert("-+3", false);
    a.insert("95a54e53", false);
    a.insert("", false);
    a.insert("+", false);
    a.insert("-", false);
    a.insert("-----", false);
    a.insert("0-----", false);
    a.insert("1 ", true);
    a.insert(".1 ", true);
    a.insert("-.1 ", true);
    a.insert("2e-.1 ", false);
    a.insert("2. ", true);
    a.insert("2.", true);
    a.insert("-2.", true);
    a.insert("-2.e", false);
    a.insert("-2.e2", true);
    a.insert("-2.e-2", true);
    a.insert("-2.e-2 ", true);
    a.insert("2e2. ", false);
    a.insert(".", false);
    a.insert(".1", true);
    a.insert(". ", false);
    a.insert("-7e", false);
    a.insert("-.", false);
    a.insert("40.81", true);
    a.insert(" 9657.91e9", true);
    a.insert("3.5e+3.5e+3.5", false);
    for (k, v) in a {
        println!("{}=>{}", k.to_string(), v);
        assert_eq!(is_number(k.to_string()), v);
    }
}
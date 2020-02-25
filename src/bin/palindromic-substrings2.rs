use std::collections::HashMap;

fn count_substrings(s: String) -> i32 {
    let l = s.len() as i32;
    if l <= 1 {
        return l;
    }
    let mut ans = 0;
    let ss = s.as_bytes();
    for i in 0..(2*l - 1) {
        let mut left = (i / 2);
        let mut right = left + (i % 2 );
        while left >= 0 && right < l && ss[left as usize] == ss[right as usize] {
            ans += 1;
            left -= 1;
            right += 1;
        }
    }
    ans
}

fn main() {
    let mut ss = HashMap::new();
    ss.insert("abc", 3);
    ss.insert("aaa", 6);
    ss.insert("aaaa", 10);
    for (k, v) in ss {
        assert_eq!(count_substrings(k.to_string()), v)
    }
}
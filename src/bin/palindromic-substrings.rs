use std::collections::HashMap;

fn count_substrings(s: String) -> i32 {
    let l = s.len();
    if l <= 1 {
        return l as i32;
    }
    let mut count = 0;
    let ss = s.as_bytes();
    for i in 0..l {
        for j in (i + 1 .. l).rev() {
            if ss[i] == ss[j] {
                let mut left = i;
                let mut right = j;
                loop {
                    if right - left == (j - i) % 2 {
                        count += 1;
                        break;
                    }
                    left += 1;
                    right -= 1;
                    if ss[left] != ss[right] {
                        break;
                    }
                }
            }
        }
    }
    count + l as i32
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
fn longest_palindrome(s: String) -> String {
    let l = s.len();
    if l <= 1 {
        return s;
    }
    let mut left = 0;
    let mut right = 0;
    let ss = s.as_bytes();
    let mut max = 0;

    for i in 0..l {
        for j in (i+1..l).rev() {
            if j - i < max {
                break;
            }
            if ss[i] == ss[j] {
                let mut last_left = i;
                let mut last_right = j;
                loop {
                    if last_right - last_left == (j - i) % 2 {
                        if max < j - i {
                            left = i;
                            right = j;
                            max = j - i;
                        }
                        break;
                    }
                    last_left += 1;
                    last_right -= 1;
                    if ss[last_right] != ss[last_left] {
                        break;
                    }
                }
            }
        }
    }
    if left == right {
        return s[0..1].parse().unwrap()
    }
    s[left..right+1].parse().unwrap()
}

fn main() {
    assert_eq!(longest_palindrome("aaabaaaa".to_string()), "aaabaaa".to_string());
    assert_eq!(longest_palindrome("".to_string()), "".to_string());
    assert_eq!(longest_palindrome("a".to_string()), "a".to_string());
    assert_eq!(longest_palindrome("ab".to_string()), "a".to_string());
    assert_eq!(longest_palindrome("aba".to_string()), "aba".to_string());
}
fn remove_palindrome_sub(s: String) -> i32 {
    let l = s.len();
    if l < 1 {
        return 0;
    } else if l == 1 {
        return 1;
    }
    let cc = s.as_bytes();
    let mut left = 0;
    let mut right = l - 1;
    while left < right {
        if cc[left] == cc[right] {
            left += 1;
            right -= 1;
        } else {
            return 2;
        }
    }
    return 1;
}

fn main() {
    assert_eq!(remove_palindrome_sub("ababa".to_string()), 1);
    assert_eq!(remove_palindrome_sub("abb".to_string()), 2);
    assert_eq!(remove_palindrome_sub("baabb".to_string()), 2);
    assert_eq!(remove_palindrome_sub("".to_string()), 0);
    assert_eq!(remove_palindrome_sub("abaaaabb".to_string()), 2);
}
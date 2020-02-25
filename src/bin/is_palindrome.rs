fn is_palindrome(mut x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut y = 0;
    while x > y {
        y = y * 10 + x % 10;
        x /= 10;
    }
    return x == y || x == y / 10;
}

fn main() {
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(9), true);
    assert_eq!(is_palindrome(10), false);
    assert_eq!(is_palindrome(1221), true);
    assert_eq!(is_palindrome(12321), true);
    assert_eq!(is_palindrome(123521), false);
    assert_eq!(is_palindrome(123321), true);
    assert_eq!(is_palindrome(1231223321), false);
}
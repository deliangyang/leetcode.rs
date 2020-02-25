fn my_atoi(str: String) -> i32 {
    let mut items = vec![];
    let mut int_start = false;
    let pow = |x, mut y| {
        let mut res = 1;
        if y == 0 {
            return res;
        }
        while y > 0 {
            res *= x;
            y -= 1;
        }
        res
    };
    let mut b = 0;
    for c in str.chars() {
        if !int_start {
            if c == ' ' && b == 0 {
                continue;
            }
            if b != 0 && (c == '-' || c == '+') {
                break;
            }
            if c == '0' && b == 0 {
                int_start = true;
                b = 1;
                continue;
            }
            if (c < '0' || c > '9') && c != '-' && c != '+' {
                break;
            }
            if c == '-' {
                b = -1;
                continue;
            } else if c == '+' {
                b = 1;
                continue;
            }
            int_start = true;
        } else {
            if c < '0' || c > '9' {
                break;
            }
        }
        if b != 0 && items.is_empty() && c == '0' {
            continue;
        }
        items.push(c);
    }
    let l = items.len();
    if l == 0 {
        return 0;
    }

    if b == 0 {
        b = 1;
    }
    if l == 1 {
        return (items[0] as i32 - 48) * b;
    }
    if l > 10 || (l >= 10 && items[0] as i32 - 48 > 2) {
        return if b == -1 {
            -2147483648
        } else {
            2147483647
        };
    }

    let mut res = 0;
    for i in 0..l {
        let y = l as i32 - i as i32 - 1;
        let current = pow(10, y);
        let num = items[i] as i32 - 48;
        if b == 1 {
            if res > 2147483647 - num * current {
                return 2147483647;
            }
        } else {
            if res - 1 >= 2147483647 - num * current {
                return -2147483648;
            }
        }
        res += num * current;
    }
    res * b
}

fn main() {
    assert_eq!(my_atoi("-91283472332".to_string()), -2147483648);
    assert_eq!(my_atoi("words and 987".to_string()), 0);
    assert_eq!(my_atoi("   -42".to_string()), -42);
    assert_eq!(my_atoi("42".to_string()), 42);
    assert_eq!(my_atoi("4193 with words".to_string()), 4193);
    assert_eq!(my_atoi("-".to_string()), 0);
    assert_eq!(my_atoi("-4193".to_string()), -4193);
    assert_eq!(my_atoi("91283472332".to_string()), 2147483647);
    assert_eq!(my_atoi("  000-1".to_string()), 0);
    assert_eq!(my_atoi("  000-1122".to_string()), 0);
    assert_eq!(my_atoi("-1122 ".to_string()), -1122);
    assert_eq!(my_atoi("-1122 12".to_string()), -1122);
    assert_eq!(my_atoi("+1122 12".to_string()), 1122);
    assert_eq!(my_atoi("  0000000000012345678".to_string()), 12345678);
    assert_eq!(my_atoi("-000000000000001".to_string()), -1);
    assert_eq!(my_atoi("+000000000000001".to_string()), 1);
    assert_eq!(my_atoi("+-1".to_string()), 0);
    assert_eq!(my_atoi("+--1".to_string()), 0);
    assert_eq!(my_atoi("+++++1".to_string()), 0);
    assert_eq!(my_atoi("---1".to_string()), 0);
    assert_eq!(my_atoi("+".to_string()), 0);
    assert_eq!(my_atoi("-".to_string()), 0);
    assert_eq!(my_atoi("-".to_string()), 0);
    assert_eq!(my_atoi("   +0 123".to_string()), 0);
    assert_eq!(my_atoi("     +004500".to_string()), 4500);
    assert_eq!(my_atoi("-2147483647".to_string()), -2147483647);
    assert_eq!(my_atoi("-2147483648".to_string()), -2147483648);
    assert_eq!(my_atoi("-2147483649".to_string()), -2147483648);
    assert_eq!(my_atoi("2147483648".to_string()), 2147483647);
    assert_eq!(my_atoi("2147483649".to_string()), 2147483647);
    assert_eq!(my_atoi("2147483647".to_string()), 2147483647);
    assert_eq!(my_atoi("2147483646".to_string()), 2147483646);
    assert_eq!(my_atoi("-2147483646".to_string()), -2147483646);
    assert_eq!(my_atoi("-6147483648".to_string()), -2147483648);
}
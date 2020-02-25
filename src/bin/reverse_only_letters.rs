fn reverse_only_letters(s: String) -> String {
    let mut ns = vec![];
    let l = s.len();
    let is_letter = |c| {
        let code = c as i32;
        return if (code >= 'a' as i32 && code <= 'z' as i32)
            || (code >= 'A' as i32 && code <= 'Z' as i32)  {
            true
        } else {
            false
        }
    };
    let mut x = 0;
    let cc = s.as_bytes();
    for i in (0..l).rev() {
        while x < l {
            if !is_letter(cc[x]) {
                ns.push(cc[x]);
                x += 1;
            } else {
                break;
            }
        }

        if is_letter(cc[i]) {
            ns.push(cc[i]);
            x += 1;
        }
    }
    for i in x..l {
        ns.push(cc[i]);
    }
    String::from_utf8(ns).unwrap()
}

fn main() {
    assert_eq!(reverse_only_letters("ab-cd".to_string()), "dc-ba".to_string());
    assert_eq!(reverse_only_letters("=------".to_string()), "=------".to_string());
    assert_eq!(reverse_only_letters("".to_string()), "".to_string());
    assert_eq!(reverse_only_letters("ab-----".to_string()), "ba-----".to_string());
    assert_eq!(reverse_only_letters("-----ab".to_string()), "-----ba".to_string());
    assert_eq!(reverse_only_letters("a-bC-dEf-ghIj".to_string()), "j-Ih-gfE-dCba".to_string());
    assert_eq!(reverse_only_letters("Test1ng-Leet=code-Q!".to_string()), "Qedo1ct-eeLg=ntse-T!".to_string());
}
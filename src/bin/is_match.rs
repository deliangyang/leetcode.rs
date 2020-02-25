pub fn is_match(s: String, p: String) -> bool {
    let l = s.len();
    let lp = p.len();
    let mut dp: Vec<Vec<bool>> = Vec::with_capacity(l + 1);
    for _ in 0..l + 1 {
        let mut tmp = Vec::with_capacity(lp + 1);
        for _ in 0..lp + 1 {
            tmp.push(false)
        }
        dp.push(tmp)
    }
    dp[l][lp] = true;

    let ss = s.as_bytes();
    let sp = p.as_bytes();
    for i in (0..l + 1).rev() {
        for j in (0..lp).rev() {
            let first_match = i < l && (sp[j] == ss[i] || sp[j] == '.' as u8);
            if j + 1 < lp && sp[j + 1] == '*' as u8 {
                dp[i][j] = dp[i][j + 2] || first_match && dp[i + 1][j];
            } else {
                dp[i][j] = first_match && dp[i + 1][j + 1];
            }
        }
    }
    dp[0][0]
}

fn main() {
    println!("{}", is_match("aab".to_string(), "c*a*b".to_string()));
    println!("{}", is_match("mississippi".to_string(), "mis*is*p*".to_string()));
}
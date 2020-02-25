fn convert(s: String, num_rows: i32) -> String {
    let l = s.len();
    if l <= 0 || num_rows < 1 {
        return "".to_string()
    }
    if num_rows == 1 {
        return s
    }
    let mut cs:Vec<Vec<char>> = vec![];
    let ss = s.as_bytes();

    for _ in 0..num_rows {
        cs.push(vec![])
    }

    let n = num_rows as usize;

    for i in 0..l {
        let ans = i / (n - 1);
        let cur = i % (n - 1);
        if ans % 2 == 0 {
            cs[cur].push(char::from(ss[i]))
        } else {
            cs[n - cur - 1].push(char::from(ss[i]))
        }
    }
    let mut sb = vec![];
    for i in cs {
        for j in i {
            sb.push(j as u8)
        }
    }
    String::from_utf8(sb).unwrap()
}

fn test() -> String {
    String::from("aaa".to_string())
}

fn main() {
    println!("{}", convert("LEETCODEISHIRING".to_string(), 3));

    test();
}
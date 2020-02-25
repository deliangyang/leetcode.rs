fn int_to_roman(mut num: i32) -> String {
    let nums = vec![1, 5, 10, 50, 100, 500, 1000, ];
    let s = vec!['I', 'V', 'X', 'L', 'C', 'D', 'M', ];

    let mut chars = vec![];
    let mut count = 0;
    while num / 10 > 0 {
        let mut n = num % 10;
        if n < 5 {
            chars.insert(0,s[count + 1]);
            while  n > 0 {
                n -= 1;
            }
        }
        num /= 10;
    }
    String::from_utf8(chars).unwrap()

}

fn main() {
    assert_eq!(int_to_roman(3), "III");
    assert_eq!(int_to_roman(4), "IV");
    assert_eq!(int_to_roman(9), "IX");
    assert_eq!(int_to_roman(58), "LVIII");
    assert_eq!(int_to_roman(1994), "MCMXCIV");
}
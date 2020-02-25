use std::collections::HashMap;

fn respace(dictionary: Vec<String>, sentence: String) -> i32 {
    let dl = dictionary.len();

    for s in dictionary {

    }
    let l = sentence.len();
    let mut dp = vec![0;l];
    for i in 0..l {

    }
    "".to_string();

}

fn main() {
    let dictionary = vec![
        "looked".to_string(), "just".to_string(), "like".to_string(),
        "her".to_string(), "brother".to_string(), ];
    let sentence = "jesslookedjustliketimherbrother";

    assert_eq!(respace(dictionary, sentence.to_string()), 7);
}
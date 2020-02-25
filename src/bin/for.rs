fn main() {
    for i in 0..3 {
        for j in (i+1..3).rev() {
            println!("({}, {})", i, j);
        }
    }
}
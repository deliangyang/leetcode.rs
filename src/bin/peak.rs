
pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
    let l = a.len();
    if l == 1 {
        return 0
    } else if l == 2 {
        return if a[0] > a[1] {
            0
        } else {
            1
        }
    }
    let mut left = 0;
    let mut right = l - 1;
    let mut mid;
    while right - left > 1 {
        mid = left + (right - left) / 2;
        if a[mid] > a[mid - 1] && a[mid] > a[mid + 1] {
            return mid as i32
        } else if a[mid] > a[mid + 1] && a[mid - 1] > a[mid] {
            right = mid
        } else {
            left = mid
        }
    }
    if a[left] > a[right] {
        return left as i32
    }
    right as i32
}

fn main() {
    let a = vec![3,4,5,1];
    println!("{}", peak_index_in_mountain_array(a));
}

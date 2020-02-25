use std::cmp::{min, max};

fn max_area(height: Vec<i32>) -> i32 {
    let l = height.len();
    let mut max_area = 0;
    let mut left = 0;
    let mut right = l - 1;

    while left < right {
        max_area = max(max_area, min(height[left], height[right]) * (right - left) as i32);
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    max_area
}

fn main() {
    let area = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(max_area(area), 49);
}
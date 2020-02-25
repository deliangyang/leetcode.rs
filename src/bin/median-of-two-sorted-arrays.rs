fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
    let l1 = nums1.len();
    let l2 = nums2.len();
    let mid = (l1 + l2) / 2;
    let md = (l1 + l2) % 2;
    if l1 == 0 {
        return if md == 0 {
            (nums2[mid - 1] as f64 + nums2[mid] as f64) / 2.0
        } else {
            nums2[mid] as f64
        };
    } else if l2 == 0 {
        return if md == 0 {
            (nums1[mid - 1] as f64 + nums1[mid] as f64) / 2.0
        } else {
            nums1[mid] as f64
        };
    }
    nums1.append(nums2.as_mut());
    nums1.sort();
    return if md == 0 {
        (nums1[mid-1] as f64 + nums1[mid] as f64) / 2.0
    } else {
        nums1[mid] as f64
    };
}

fn main() {
    assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![]), 1.5);
    assert_eq!(find_median_sorted_arrays(vec![], vec![1, 2]), 1.5);
}
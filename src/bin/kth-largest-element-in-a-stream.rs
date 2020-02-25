struct KthLargest {
    l: usize,
    stack: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        let l = k as usize;
        if nums.len() >= l {
            nums.sort_by(|a, b| b.cmp(a));
            nums = nums[0..l].to_owned();
        }
        KthLargest {
            l,
            stack: nums,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.stack.push(val);
        self.stack.sort_by(|a, b| b.cmp(a));
        self.stack = self.stack[0..self.l].to_owned();
        *self.stack.last().unwrap()
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

fn main() {
    let mut obj = KthLargest::new(2, vec![0]);
    assert_eq!(obj.add(-1), -1);
    assert_eq!(obj.add(1), 0);
    assert_eq!(obj.add(-2), 0);
    assert_eq!(obj.add(-4), 0);
    assert_eq!(obj.add(3), 1);
}
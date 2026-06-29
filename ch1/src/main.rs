struct Solution;
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut result = Vec::with_capacity(2 * n);

        for i in 0..n {
            result.push(nums[i]);
            result.push(nums[i + n]);
        }
        result
    }
}
fn main() {
    let nums = vec![1, 2, 3, 4];
    let n: Vec<i32> = Solution::shuffle(nums, 2);
    println!("{:?}", n);
}

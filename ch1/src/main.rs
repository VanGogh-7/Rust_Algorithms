struct Solution;

impl Solution {
    pub fn get_concatenation(nums: &Vec<i32>) -> Vec<i32> {
        let mut ans = nums.clone();
        ans.extend(nums);
        ans
    }

    pub fn shuffle(nums: &Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut result = Vec::with_capacity(2 * n);
        for i in 0..n {
            result.push(nums[i]);
            result.push(nums[i + n]);
        }
        result
    }

    pub fn find_max_consecutive(nums: &Vec<i32>) -> i32 {
        nums.split(|&num| num == 0)
            .map(|sub_slice| sub_slice.len())
            .max()
            .unwrap_or(0) as i32
    }
}

fn main() {
    let nums0: Vec<i32> = vec![1, 2, 3, 4];
    let nums1: Vec<i32> = vec![0, 1, 1, 1];
    let sol = Solution::get_concatenation(&nums0);
    println!("solution: {:?}", sol);

    let sol = Solution::shuffle(&nums0, 2);
    println!("solution: {:?}", sol);

    let sol = Solution::find_max_consecutive(&nums1);
    println!("solution: {:?}", sol);
}

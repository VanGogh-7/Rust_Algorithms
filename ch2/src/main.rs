struct Solution;
impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let mut duplicated = 0;
        let mut missing = 0;
        for i in 0..nums.len() {
            let index = nums[i].abs() as usize - 1;
            if nums[index] < 0 {
                duplicated = index as i32 + 1;
            } else {
                nums[index] = -nums[index];
            }
        }

        for i in 0..nums.len() {
            if nums[i] > 0 {
                missing = i as i32 + 1;
                break;
            }
        }
        vec![duplicated, missing]
    }

    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        nums.into_iter()
            .map(|num| sorted.partition_point(|&x| x < num) as i32)
            .collect()
    }

    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let index = (nums[i].abs() - 1) as usize;
            if nums[index] > 0 {
                nums[index] = -nums[index];
            }
        }
        nums.into_iter()
            .enumerate()
            .filter(|&(_, x)| x > 0)
            .map(|(index, _)| (index + 1) as i32)
            .collect()
    }
}

fn main() {
    let nums0 = vec![1, 2, 2, 4];
    println!("{:?}", Solution::find_error_nums(nums0));

    let nums1 = vec![8, 3, 6, 5, 4];
    println!("{:?}", Solution::smaller_numbers_than_current(nums1));

    let num2 = vec![4, 3, 2, 2];
    println!("{:?}", Solution::find_disappeared_numbers(num2));
}

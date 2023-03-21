// https://leetcode.com/problems/separate-the-digits-in-an-array

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        for n in nums.iter() {
            let mut string: String = n.to_string();
            for c in string.chars() {
                ans.push((c as u8 - b'0') as i32);
            }
        }
        ans
    }
}

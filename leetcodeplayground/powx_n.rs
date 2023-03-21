// https://leetcode.com/problems/powx-n

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0.0f64 {
            0.0
        } else if x == 1.0f64 || n == 0i32 {
            1.0
        } else if x == -1.0f64 {
            if n % 2 == 0 {
                1.0f64
            } else {
                -1.0f64
            }
        } else if n == i32::MIN || n == i32::MAX {
            0.0
        } else {
            let mut result: f64 = x;
            let mut i: i32 = 1;
            while i < n.abs() {
                result = result * x;
                i += 1;
            }
            if n > 0 {
                return result;
            } else if n < 0 {
                return 1.0f64 / result;
            } else {
                1.0
            }
        }
    }
}

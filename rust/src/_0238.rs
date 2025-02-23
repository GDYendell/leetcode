/*
* Given an integer array `nums`, return *an array* `answer` *such that*
* `answer[i]` *is equal to the product of all the elements of* `nums` *except*
* `nums[i]`.
*
* The product of any prefix or suffix of `nums` is guaranteed to fit in a
* 32-bit integer.
*
* You must write an algorithm that runs in `O(n)` time and without using the
* division operation.
*
*
* Example 1:
*
* Input: nums = [1,2,3,4]
* Output: [24,12,8,6]
*
* Example 2:
*
* Input: nums = [-1,1,0,-3,3]
* Output: [0,0,9,0,0]
*
*
* Constraints:
*
* * `2 <= nums.length <= 105`
* * `-30 <= nums[i] <= 30`
* * The input is generated such that `answer[i]` is guaranteed to fit in a
*   32-bit integer.
*
*
* Follow up: Can you solve the problem in `O(1)` extra space complexity? (The
* output array does not count as extra space for space complexity analysis.)
*
*/

pub struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];
        let mut accumulator = 1;

        // Iterate forwards accumulating the product of values
        // Multiply the next element to the right by the accumulation
        // e.g. [a, b, c, d, e]: [1, a, ab, abc, abcd]
        for i in 0..(nums.len() - 1) {
            accumulator *= nums[i];
            result[i + 1] *= accumulator;
        }

        // Iterate backwards accumulating the product of values
        // Multiply the next element to the left by the accumulation
        // e.g. [a, b, c, d, e]: [bcde, cde, de, e, 1]
        // [1, a, ab, abc, abcd] -> [1*bcde, a*cde, ab*de, abc*e, abcd*1]
        accumulator = 1;
        for i in (1..nums.len()).rev() {
            accumulator *= nums[i];
            result[i - 1] *= accumulator;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}

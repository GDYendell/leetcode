/*
* Given an integer array `nums`, return `true`* if there exists a triple of
* indices *`(i, j, k)`* such that *`i < j < k`* and *`nums[i] < nums[j] <
* nums[k]`. If no such indices exists, return `false`.
*
*
* Example 1:
*
* Input: nums = [1,2,3,4,5]
* Output: true
* Explanation: Any triplet where i < j < k is valid.
*
* Example 2:
*
* Input: nums = [5,4,3,2,1]
* Output: false
* Explanation: No triplet exists.
*
* Example 3:
*
* Input: nums = [2,1,5,0,4,6]
* Output: true
* Explanation: The triplet (3, 4, 5) is valid because nums[3] == 0 < nums[4] =
* = 4 < nums[5] == 6.
*
*
* Constraints:
*
* * `1 <= nums.length <= 5 * 105`
* * `-231 <= nums[i] <= 231 - 1`
*
*
* Follow up: Could you implement a solution that runs in `O(n)` time
* complexity and `O(1)` space complexity?
*
*/
use std::i32;

pub struct Solution;
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first = i32::MAX;
        let mut second = i32::MAX;

        for n in nums {
            if n <= first {
                first = n;
            } else if n <= second {
                second = n;
            } else {
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::increasing_triplet(vec![5, 4, 3, 2, 1]));
    }

    #[test]
    fn test_3() {
        assert!(Solution::increasing_triplet(vec![1, 5, 0, 4, 1, 3]));
    }

    #[test]
    fn test_4() {
        assert!(Solution::increasing_triplet(vec![1, 2, 2147483647]));
    }
}

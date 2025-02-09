/*
* You have a long flowerbed in which some of the plots are planted, and some are
* not. However, flowers cannot be planted in adjacent plots.
*
* Given an integer array `flowerbed` containing `0`'s and `1`'s, where `0` means
* empty and `1` means not empty, and an integer `n`, return `true` *if* `n` *new
* flowers can be planted in the* `flowerbed` *without violating the
* no-adjacent-flowers rule and* `false` *otherwise*.
*
*
* Example 1:
*
* Input: flowerbed = [1,0,0,0,1], n = 1
* Output: true
*
* Example 2:
*
* Input: flowerbed = [1,0,0,0,1], n = 2
* Output: false
*
*
* Constraints:
*
* * `1 <= flowerbed.length <= 2 * 104`
* * `flowerbed[i]` is `0` or `1`.
* * There are no two adjacent flowers in `flowerbed`.
* * `0 <= n <= flowerbed.length`
*
*/

#![allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut available_plots = 0;
        let mut placed_previous = false;

        for (idx, x) in flowerbed.iter().enumerate() {
            if placed_previous {
                placed_previous = false;
                continue;
            }

            if x == &0
                && (idx == 0 || flowerbed[idx - 1] == 0)
                && (idx == flowerbed.len() - 1 || flowerbed[idx + 1] == 0)
                && !placed_previous
            {
                available_plots += 1;
                placed_previous = true;
            }

            if available_plots == n {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2));
    }

    #[test]
    fn test_4() {
        assert!(Solution::can_place_flowers(vec![0, 0, 1, 0, 1], 1));
    }
}

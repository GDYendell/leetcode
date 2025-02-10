/*
* Given a string `s`, reverse only all the vowels in the string and return it.
*
* The vowels are `'a'`, `'e'`, `'i'`, `'o'`, and `'u'`, and they can appear in
* both lower and upper cases, more than once.
*
*
* Example 1:
*
* Input: s = "IceCreAm"
*
* Output: "AceCreIm"
*
* Explanation:
*
* The vowels in `s` are `['I', 'e', 'e', 'A']`. On reversing the vowels, s becomes
* `"AceCreIm"`.
*
* Example 2:
*
* Input: s = "leetcode"
*
* Output: "leotcede"
*
*
* Constraints:
*
* * `1 <= s.length <= 3 * 105`
* * `s` consist of printable ASCII characters.
*
*/

pub struct Solution;
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels = "aeiouAEIOU";

        let mut s_indexes: Vec<usize> = vec![];
        let mut s_vowels: Vec<char> = vec![];

        for (idx, c) in s.chars().enumerate() {
            if vowels.contains(c) {
                s_indexes.push(idx);
                s_vowels.push(c);
            }
        }
        s_vowels.reverse();

        let mut new_s = s.clone();
        for (&idx, c) in s_indexes.iter().zip(s_vowels.iter()) {
            new_s.replace_range(idx..idx + 1, &c.to_string());
        }

        new_s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::reverse_vowels("IceCreAm".to_string()),
            "AceCreIm".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::reverse_vowels("leetcode".to_string()),
            "leotcede".to_string()
        );
    }
}

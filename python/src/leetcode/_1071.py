# For two strings `s` and `t`, we say "`t` divides `s`" if and only if `s = t + t
# + t + ... + t + t` (i.e., `t` is concatenated with itself one or more times).
#
# Given two strings `str1` and `str2`, return *the largest string *`x`* such that
# *`x`* divides both *`str1`* and *`str2`.
#
#
# Example 1:
#
# Input: str1 = "ABCABC", str2 = "ABC"
# Output: "ABC"
#
# Example 2:
#
# Input: str1 = "ABABAB", str2 = "ABAB"
# Output: "AB"
#
# Example 3:
#
# Input: str1 = "LEET", str2 = "CODE"
# Output: ""
#
#
# Constraints:
#
# * `1 <= str1.length, str2.length <= 1000`
# * `str1` and `str2` consist of English uppercase letters.


class Solution:
    def gcdOfStrings(self, str1: str, str2: str) -> str:
        if len(str1) >= len(str2):
            return self._gcd_of_strings(str1, str2)
        else:
            return self._gcd_of_strings(str2, str1)

    def _gcd_of_strings(self, longer: str, shorter: str) -> str:
        for i in reversed(range(1, len(shorter) + 1)):
            divisor = shorter[:i]
            if self._is_divisible(longer, divisor) and self._is_divisible(
                shorter, divisor
            ):
                return divisor

        return ""

    def _is_divisible(self, s: str, divisor: str) -> bool:
        test = divisor
        while len(test) <= len(s):
            if test == s:
                return True

            test += divisor

        return False

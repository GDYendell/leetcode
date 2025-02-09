from leetcode._1768 import Solution


def test_1():
    assert Solution().mergeAlternately("abc", "pqr") == "apbqcr"


def test_2():
    assert Solution().mergeAlternately("ab", "pqrs") == "apbqrs"


def test_3():
    assert Solution().mergeAlternately("abcd", "pq") == "apbqcd"

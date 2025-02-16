from leetcode._0151 import Solution


def test_1():
    assert Solution().reverseWords("the sky is blue") == "blue is sky the"


def test_2():
    assert Solution().reverseWords("  hello world  ") == "world hello"


def test_3():
    assert Solution().reverseWords("a good   example") == "example good a"

from leetcode._1071 import Solution


def test_1():
    assert Solution().gcdOfStrings("ABCABC", "ABC") == "ABC"


def test_2():
    assert Solution().gcdOfStrings("ABABAB", "AB") == "AB"


def test_3():
    assert Solution().gcdOfStrings("LEET", "CODE") == ""


def test_4():
    assert (
        Solution().gcdOfStrings(
            "TAUXXTAUXXTAUXXTAUXXTAUXX", "TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX"
        )
        == "TAUXX"
    )

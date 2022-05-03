from typing import Iterable, Optional


def find_outlier(integers: Iterable[int]) -> Optional[int]:
    """Find The Parity Outlier.

    Idea: go though array, counting odd and even integers, counting numnber of
    odd and even integers seen, and saving the last seen.

    We can terminate early when we have >= 2 odd and one even (which will be
    our outlier) or vice verse.
    """
    last_odd = None
    num_odd = 0
    last_even = None
    num_even = 0
    for i in integers:
        if i % 2 == 0:
            last_even = i
            num_even += 1
        else:
            last_odd = i
            num_odd += 1
        # Did we find even outlier?
        if num_even == 1 and num_odd >= 2:
            return last_even
        # Or is it odd outlier?
        elif num_odd == 1 and num_even >= 2:
            return last_odd
    # This should never happen if passed array is in correct format.
    return None


if __name__ == '__main__':
    assert find_outlier([2, 4, 6, 8, 10, 3]) == 3
    assert find_outlier([2, 4, 0, 100, 4, 11, 2602, 36]) == 11
    assert find_outlier([160, 3, 1719, 19, 11, 13, -21]) == 160
    print("passed")

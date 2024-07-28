from typing import List
from collections import Counter


class Solution:
    def frequencySort(self, nums: List[int]) -> List[int]:
        freq: Counter = Counter(nums)
        return sorted(nums, key=lambda x: (freq[x], -x))


def test(nums: List[int], expected: List[int]) -> None:
    result: List[int] = Solution().frequencySort(nums)
    if result == expected:
        print(f" + Test 'FS({nums}) == {expected}' passed!")
    else:
        print(f" - Test 'FS({nums}) == {expected}' failed with value {result}!")


if __name__ == "__main__":
    test([1, 1, 2, 2, 2, 3], [3, 1, 1, 2, 2, 2])
    test([2, 3, 1, 3, 2], [1, 3, 3, 2, 2])
    test([-1, 1, -6, 4, 5, -6, 1, 4, 1], [5, -1, 4, 4, -6, -6, 1, 1, 1])

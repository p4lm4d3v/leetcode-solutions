from typing import List


class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        curr_sum: int = sum(nums)
        expected: int = int(len(nums) * (len(nums) + 1) / 2)
        return expected - curr_sum


def missingNumberTest(nums: List[int], solution: int) -> None:
    result: int = Solution().missingNumber(nums)
    if result == solution:
        print(f" + 'Test ML({nums}) == {solution}' passed!")
    else:
        print(f" - 'Test ML({nums}) == {solution}' failed with value {result}!")


if __name__ == "__main__":
    missingNumberTest([3, 0, 1], 2)
    missingNumberTest([0, 1], 2)
    missingNumberTest([9, 6, 4, 2, 3, 5, 7, 0, 1], 8)

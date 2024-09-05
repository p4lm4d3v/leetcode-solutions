from typing import List


class Solution:
    def maximumGap(self, nums: List[int]) -> int:
        if len(nums) < 2:
            return 0

        nums.sort()

        maximum: int = 0
        for i in reversed(range(1, len(nums))):
            maximum = max(maximum, nums[i] - nums[i - 1])

        return maximum


def maximumGapTest(nums: List[int], expected: int) -> None:
    result: int = Solution().maximumGap(nums)
    if result == expected:
        print(f" + Test 'MaximumGap({nums}) == {expected}' passed!")
    else:
        print(f" - Test 'MaximumGap({nums}) == {expected}' failed with value {result}!")


if __name__ == "__main__":
    maximumGapTest([3, 6, 9, 1], 3)
    maximumGapTest([10], 0)

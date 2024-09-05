from typing import List, Self, Set


class Solution:
    def longestConsecutive(self: Self, numsList: List[int]) -> int:
        numsSet: Set = set(numsList)
        longest: int = 0

        for n in numsSet:
            if (n - 1) not in numsSet:
                length: int = 1

                while (n + length) in numsSet:
                    length += 1

                longest = max(longest, length)

        return longest


def longestConsecutiveTest(nums: List[int], solution: int) -> None:
    result: int = Solution().longestConsecutive(nums)
    if result == solution:
        print(f" + Test 'longestConsecutive({nums}) == {solution}' passed!")
    else:
        print(
            f" - Test 'longestConsecutive({nums}) == {solution}' failed with value {result}!"
        )


if __name__ == "__main__":
    longestConsecutiveTest([100, 4, 200, 1, 3, 2], 4)
    longestConsecutiveTest([0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9)

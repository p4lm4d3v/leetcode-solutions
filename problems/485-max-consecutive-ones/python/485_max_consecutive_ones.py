from typing import List


class Solution:
    def findMaxConsecutiveOnes(self, nums: List[int]) -> int:
        max: int = 0
        counter: int = 0

        nums.append(0);

        for n in nums:
            if n == 1:
                counter += 1
            elif n == 0:
                if counter > max:
                    max = counter
                counter = 0

        return max


def test(nums: List[int], expected: int) -> None:
    result: str = Solution().findMaxConsecutiveOnes(nums)
    if result == expected:
        print(f" + Test 'FindMaxConsecutive({nums}) == {expected}' passed!")
    else:
        print(
            f" - Test 'FindMaxConsecutiveOnes({nums}) == {expected}' failed with value {result}!"
        )


if __name__ == "__main__":
    test([1,1,0,1,1,1], 3)
    test([1,0,1,1,0,1], 2)


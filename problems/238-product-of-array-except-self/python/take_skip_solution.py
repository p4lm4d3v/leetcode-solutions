from functools import reduce
from itertools import islice
from typing import List


class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        result: List[int] = [0 for _ in nums]

        for i in range(1, len(nums) + 1):
            leftSide: List[int] = list(islice(nums, i - 1))
            left: int = reduce(lambda a, b: a * b, leftSide, 1)

            rightSide: List[int] = list(islice(nums, i, None))
            right: int = reduce(lambda a, b: a * b, rightSide, 1)

            result[i - 1] = left * right

        return result


def test(nums: List[int]) -> None:
    result: int = Solution().productExceptSelf(nums)
    print(f"Product except self for these numbers is {
          result} \n -> numbers: {nums}")


if __name__ == "__main__":
    test([1, 2, 3, 4])
    test([-1, 1, 0, -3, 3])

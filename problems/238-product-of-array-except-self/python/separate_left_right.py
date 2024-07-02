from typing import List


class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        result: List[int] = [0 for _ in nums]

        for i in range(len(nums)):
            left: int = 1
            for l in range(0, i):
                left *= nums[l]

            right: int = 1
            for r in range(i + 1, len(nums)):
                right *= nums[r]

            result[i] = left * right

        return result


def test(nums: List[int]) -> None:
    result: int = Solution().productExceptSelf(nums)
    print(f"Product except self for these numbers is {
          result} \n -> numbers: {nums}")


if __name__ == "__main__":
    test([1, 2, 3, 4])
    test([-1, 1, 0, -3, 3])

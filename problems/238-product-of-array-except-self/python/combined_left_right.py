from typing import List


class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        result: List[int] = [0 for _ in nums]

        for i in range(len(nums)):
            left: int = 1
            right: int = 1

            for j in range(len(nums)):
                n: int = nums[j]
                if j < i:
                    left *= n
                elif j > i:
                    right *= n

            result[i] = left * right

        return result


def test(nums: List[int]) -> None:
    result: int = Solution().productExceptSelf(nums)
    print(f"Product except self for these numbers is {
          result} \n -> numbers: {nums}")


if __name__ == "__main__":
    test([1, 2, 3, 4])
    test([-1, 1, 0, -3, 3])

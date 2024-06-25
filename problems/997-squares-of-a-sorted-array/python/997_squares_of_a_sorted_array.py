from typing import List


class Solution:
    def sortedSquares(self, nums: List[int]) -> List[int]:
        nums = [n * n for n in nums]
        nums.sort()
        return nums


def test(nums) -> None:
    squares = Solution().sortedSquares(nums)
    print(f"Squares array for {nums} is {squares}")


if __name__ == "__main__":
    test([-4, -1, 0, 3, 10])
    test([-7, -3, 2, 3, 11])

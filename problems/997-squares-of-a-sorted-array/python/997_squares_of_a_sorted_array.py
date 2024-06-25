from typing import List


class Solution:
    def sortedSquares(self, nums: List[int]) -> List[int]:
        nums: List[int} = [n * n for n in nums]
        nums.sort()
        return nums


def test(nums: List[int]) -> None:
    squares: List[int] = Solution().sortedSquares(nums)
    print(f"Squares array for {nums} is {squares}")


if __name__ == "__main__":
    test(nums=[-4, -1, 0, 3, 10])
    test(nums=[-7, -3, 2, 3, 11])

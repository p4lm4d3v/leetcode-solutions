from typing import List, Dict


class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        result: int = 0
        for n in nums:
            result ^= n
        return result


def test(nums: List[int]) -> None:
    x: int = Solution().singleNumber(nums)
    print(f"The single number of {nums} is {x}")


if __name__ == "__main__":
    test([2, 2, 1])
    test([4, 1, 2, 1, 2])
    test([1])

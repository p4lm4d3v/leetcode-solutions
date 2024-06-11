from typing import Dict, List


class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        dict: Dict[int, int] = {}
        for n in nums:
            if n in dict:
                dict[n] += 1
            else:
                dict[n] = 1
        for k, v in dict.items():
            if v == 1:
                return k


def test(nums: List[int]) -> None:
    x: int = Solution().singleNumber(nums)
    print(f"The single number of {nums} is {x}")


if __name__ == "__main__":
    test([2, 2, 3, 2])
    test([0, 1, 0, 1, 0, 1, 99])

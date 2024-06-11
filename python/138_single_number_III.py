from typing import Dict, List


class Solution:
    def singleNumber(self, nums: List[int]) -> List[int]:
        dict: Dict[int, int] = {}
        for n in nums:
            if n in dict:
                dict[n] += 1
            else:
                dict[n] = 1
        singles: List[int] = []
        for k, v in dict.items():
            if v == 1:
                singles.append(k)
        return singles


def test(nums: List[int]) -> None:
    r: List[int] = Solution().singleNumber(nums)
    w1: str = 'are' if len(r) > 1 else 'is'
    w2: str = 'number' + 's' if len(r) > 1 else ''
    print(f"The single {w1} of {nums} {w2} {r}")


if __name__ == "__main__":
    test([1, 2, 1, 3, 2, 5])
    test([-1, 0])
    test([0, 1])

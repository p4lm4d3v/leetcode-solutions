# Explanation in this video
# url: https://www.instagram.com/reel/C4i1P5gqai6/?igsh=MW91d2NwbXRidnIxeg%3D%3D

from functools import reduce
from typing import List


class Solution:
    def minimumBoxes(self, apples: List[int], capacity: List[int]) -> int:
        capacity.sort(reverse=True)
        total_apples: int = 0
        used_boxes: int = 0

        for apple in apples:
            total_apples += apple

        while total_apples > 0:
            c: int = capacity[0]
            capacity.pop(0)
            total_apples -= c
            used_boxes += 1

        return used_boxes


def test(apple: List[int], capacity: List[int]) -> None:
    c = capacity.copy()
    r: int = Solution().minimumBoxes(apple, capacity)
    print(f"apple: {apple} fills {r} boxes from capacity: {c}")


if __name__ == "__main__":
    test([1, 3, 2], [4, 3, 1, 5, 2])
    test([5, 5, 5], [2, 4, 2, 7])
    test([2, 1, 4, 3], [5, 1, 8])

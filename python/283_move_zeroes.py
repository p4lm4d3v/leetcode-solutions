from typing import List


def examples(lists: List[List[int]]) -> None:
    for idx, list in enumerate(lists):
        print(f"Example {idx}: ")
        print(f"Input: {list}")
        Solution().moveZeroes(list)
        print(f"Output: {list}")
        if idx != len(lists) - 1:
            print()


class Solution:
    def moveZeroes(self, nums: List[int]) -> List[int]:
        count = 0
        while nums.__contains__(0):
            nums.remove(0)
            count += 1

        nums.extend([0] * count)


if __name__ == "__main__":
    examples([
        [0, 1, 0, 3, 12],
        [0],
        [0, 0, 1]
    ])

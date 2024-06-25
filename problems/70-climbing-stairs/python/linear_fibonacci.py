from typing import List


class Solution:
    def climbStairs(self, n: int, cache: dict[int, int] = {}) -> int:
        if n == 1:
            return 1
        if n == 2:
            return 2

        vec: List[int] = [1, 2]
        idx: int = 0
        for i in range(2, n):
            two_back: int = vec[i - 2]
            one_back: int = vec[i - 1]
            vec.append(two_back + one_back)
            idx = i
        return vec[idx]


def test(n: int) -> None:
    x: int = Solution().climbStairs(n)
    print(f"For {n} steps the is {x} ways to climb them!")


if __name__ == "__main__":
    for i in range(1, 11):
        test(n=i)

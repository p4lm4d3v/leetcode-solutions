from functools import cache


class Solution:
    @cache
    def climbStairs(self, n: int) -> int:
        if n == 1:
            return 1
        if n == 2:
            return 2

        two_back: int = self.climbStairs(n - 2)
        one_back: int = self.climbStairs(n - 1)
        return two_back + one_back


def test(n: int) -> None:
    x: int = Solution().climbStairs(n)
    print(f"For {n} steps the is {x} ways to climb them!")


if __name__ == "__main__":
    for i in range(1, 11):
        test(n=i)

from typing import List


class Solution:
    def fib(self, n: int, cache: dict[int, int] = {}) -> int:
        if n == 0:
            return 0
        if n == 1:
            return 1

        vec: List[int] = [1, 1]
        for i in range(2, n):
            two_back: int = vec[i - 2]
            one_back: int = vec[i - 1]
            vec.append(two_back + one_back)
        return vec[n - 1]


def test(n: int) -> None:
    x: int = Solution().fib(n)
    print(f"F({n}) = {x}")


if __name__ == "__main__":
    for i in range(0, 11):
        test(n=i)

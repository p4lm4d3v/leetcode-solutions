class Solution:
    def fib(self, n: int, cache: dict[int, int] = {}) -> int:
        if n == 0:
            return 0
        if n == 1:
            return 1

        if n in cache.keys():
            return cache[n]

        two_back: int = self.fib(n - 2, cache)
        one_back: int = self.fib(n - 1, cache)
        cache[n] = two_back + one_back
        return two_back + one_back


def test(n: int) -> None:
    x: int = Solution().fib(n)
    print(f"F({n}) = {x}")


if __name__ == "__main__":
    for i in range(0, 11):
        test(n=i)

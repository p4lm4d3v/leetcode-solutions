class Solution:
    def mySqrt(self, x: int) -> int:
        i: int = 0

        while i ** 2 < x:
            if (i + 1) ** 2 > x:
                break
            i += 1

        return i


def test(x: int) -> int:
    r: int = Solution().mySqrt(x)
    print(f"MySqrt({x}) = {r}")


if __name__ == "__main__":
    test(4)
    test(8)
    test(2147395599)
    test(2147395600)

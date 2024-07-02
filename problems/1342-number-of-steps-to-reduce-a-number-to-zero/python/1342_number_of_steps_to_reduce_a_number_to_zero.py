class Solution:
    def numberOfSteps(self, num: int) -> int:
        counter: int = 0
        while num != 0:
            if num % 2 == 0:
                num /= 2
            else:
                num -= 1
            counter += 1
        return counter


def test(num: int) -> None:
    result: int = Solution().numberOfSteps(num)
    print(f"Recutions to zero for {num} is {result}!")


if __name__ == "__main__":
    test(14)
    test(8)
    test(123)

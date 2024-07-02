from typing import List, Self


class Solution:
    def decrypt(self: Self, code: List[int], k: int) -> List[int]:
        length: int = len(code)
        result: List[int] = [0 for _ in code]

        if k == 0:
            return result

        if k < 0:
            code.reverse()

        for i in range(length):
            result[i] = self.sumK(code, i, abs(k))

        if k < 0:
            result.reverse()

        return result

    def loopIdx(self: Self, idx: int, len: int) -> int:
        return (idx + len) % len

    def sumK(self: Self, code: List[int], i: int, k: int) -> int:
        result: int = 0

        for j in range(k):
            result += code[self.loopIdx(i + 1 + j, len(code))]

        return result


def test(code: List[int], k: int) -> None:
    result: List[int] = Solution().decrypt(code, k)
    print(f"Decrypted code for this code and k is {
          result}! \n -> code: {code} \n -> k: {k}")


if __name__ == "__main__":
    test([5, 7, 1, 4], 3)
    test([1, 2, 3, 4], 0)
    test([2, 4, 9, 3], -2)

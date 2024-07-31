from typing import List


class Solution:
    def convert(self, s: str, rows: int) -> str:
        if rows == 1:
            return s

        n: int = len(s)
        matrix: List[List[chr]] = [[] for _ in range(min(rows, n))]

        zigzag: bool = False
        row: int = 0

        for c in s:
            matrix[row].append(c)

            if row == 0 or row == rows - 1:
                zigzag = not zigzag

            row += 1 if zigzag else -1

        result: List[chr] = []

        for r in matrix:
            result.extend(r)

        return "".join(result)


def test(s: str, rows: int, expected) -> None:
    result: str = Solution().convert(s, rows)
    if result == expected:
        print(f" + Test 'Convert({s}, {rows}) == {expected}' passed!")
    else:
        print(
            f" - Test 'Convert({s}, {rows}) == {expected}' failed with value {result}!"
        )


if __name__ == "__main__":
    test("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR")
    test("PAYPALISHIRING", 4, "PINALSIGYAHRPI")
    test("A", 1, "A")
    test("ABC", 2, "ACB")
    test("AB", 2, "AB")

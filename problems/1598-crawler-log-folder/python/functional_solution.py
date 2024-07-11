from typing import List, Self


class Solution:
    def minOperations(self: Self, logs: List[str]) -> int:
        vals: List[int] = [
            -1 if log == "../" else 1 if log != "./" else 0 for log in logs
        ]

        return max(0, sum(vals))


def minOperationsTest(logs: List[str], solution: int) -> None:
    result: int = Solution().minOperations(logs)
    if result == solution:
        print(f" + 'Test MO({logs}] == {solution}' passed!")
    else:
        print(f" + 'Test MO({logs}] == {solution}' failed with value {result}!")


if __name__ == "__main__":
    minOperationsTest(["d1/", "d2/", "../", "d21/", "./"], 2)
    minOperationsTest(["d1/", "d2/", "./", "d3/", "../", "d31/"], 3)
    minOperationsTest(["d1/", "../", "../", "../"], 0)

from typing import List, Self


class Solution:
    def minOperations(self: Self, logs: List[str]) -> int:
        counter: int = 0

        for log in logs:
            if log == "../":
                counter = max(0, counter - 1)
            elif log != "./":
                counter += 1

        return counter


def minOperationsTest(logs: List[str], solution: int) -> None:
    result: int = Solution().minOperations(logs)
    if result == solution:
        print(f" + 'Test MO({logs}] == {solution}' passed!")
    else:
        print(f" + 'Test MO({logs}] == {solution}' failed wiht value {result}!")


if __name__ == "__main__":
    minOperationsTest(["d1/", "d2/", "../", "d21/", "./"], 2)
    minOperationsTest(["d1/", "d2/", "./", "d3/", "../", "d31/"], 3)
    minOperationsTest(["d1/", "../", "../", "../"], 0)

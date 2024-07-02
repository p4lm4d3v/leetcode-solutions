class Solution:
    def maxDepth(self, s: str) -> int:
        max: int = 0
        depth: int = 0
        for c in s:
            if c == '(':
                depth += 1
            elif c == ")":
                depth -= 1
            if depth > max:
                max = depth
        return max


def test(s: str) -> None:
    result: int = Solution().maxDepth(s)
    print(f"Max depth for these parantheses is {
          result}!\n -> parantheses: {s}")


if __name__ == "__main__":
    test("(1+(2*3)+((8)/4))+1")
    test("(1)+((2))+(((3)))")
    test("()(())((()()))")

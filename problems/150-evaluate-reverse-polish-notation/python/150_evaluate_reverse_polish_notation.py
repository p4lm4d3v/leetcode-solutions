from typing import List, Set


class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        stack: List[int] = []
        operators: Set[str] = {"+", "-", "*", "/"}

        for token in tokens:
            if token not in operators:
                stack.append(int(token))
            else:
                b: int = stack.pop()
                a: int = stack.pop()
                match token:
                    case "+":
                        stack.append(int(a + b))
                    case "-":
                        stack.append(int(a - b))
                    case "*":
                        stack.append(int(a * b))
                    case "/":
                        stack.append(int(a / b))

        return stack.pop()


def test(tokens: List[str]) -> None:
    result: int = Solution().evalRPN(tokens)
    print(f"Value for these tokens is {result}! \n -> tokens: {tokens}")


if __name__ == "__main__":
    print()
    test(["2", "1", "+", "3", "*"])
    print()
    test(["4", "13", "5", "/", "+"])
    print()
    test(["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"])
    print()

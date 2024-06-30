class Solution:
    def lengthOfLastWord(self, s: str) -> int:
        s = s.strip()
        parsed: str = ""
        for i in reversed(range(len(s))):
            if s[i] == " ":
                break
            parsed += s[i]
        return len(parsed)


def test(s: str) -> None:
    result: int = Solution().lengthOfLastWord(s)
    print(f"Length of the last word from '{s}' is {result}!")


if __name__ == "__main__":
    test("Hello World")
    test("   fly me   to   the moon  ")
    test("luffy is still joyboy")

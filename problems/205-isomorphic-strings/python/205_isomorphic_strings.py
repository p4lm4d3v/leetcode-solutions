from typing import Dict


class Solution:
    def isIsomorphic(self, s: str, t: str) -> bool:
        if len(s) != len(t):
            return False
        if s == t:
            return True
        return self.toNumeric(s) == self.toNumeric(t)

    def toNumeric(self, s: str) -> str:
        dict: Dict[chr, int] = {}
        n: int = 1
        numeric: s = ""
        for c in s:
            if c in dict.keys():
                numeric += str(dict[c])
            else:
                dict[c] = n
                numeric += str(n)
                n += 1
        return numeric


def test(s: str, t: str) -> None:
    result: bool = Solution().isIsomorphic(s, t)
    print(f"{s} -> {t}: {result}")


if __name__ == "__main__":
    test("egg", "add")
    test("foo", "bar")
    test("paper", "title")

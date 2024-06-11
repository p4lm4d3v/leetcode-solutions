from typing import Set


class Solution:
    def isValid(self, word: str) -> bool:
        word = word.lower()
        rule1 = Solution().length_valid(word)
        rule2 = Solution().is_only_alphanumeric(word)
        rule3 = Solution().contains_vowel(word)
        rule4 = Solution().contains_consonant(word)
        return rule1 and rule2 and rule3 and rule4

    def length_valid(self, word: str) -> bool:
        return len(word) >= 3

    def is_only_alphanumeric(self, word: str) -> bool:
        return word.isalnum()

    def contains_vowel(self, word: str) -> bool:
        for letter in word:
            if Solution().is_vowel(letter):
                return True
        return False

    def contains_consonant(self, word: str) -> bool:
        for letter in word:
            if Solution().is_consonant(letter):
                return True
        return False

    def is_vowel(self, letter: str) -> bool:
        vowels: Set[str] = {"a", "e", "i", "o", "u"}
        return letter in vowels

    def is_consonant(self, letter: str) -> bool:
        consonants: Set[str] = {'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k',
                                'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z'}
        return letter in consonants


def test(word: str) -> None:
    length_valid = Solution().length_valid(word)
    is_only_alphanumeric = Solution().is_only_alphanumeric(word)
    contains_vowel = Solution().contains_vowel(word)
    contains_consonant = Solution().contains_consonant(word)
    result = Solution().isValid(word)
    _not = " not" if not result else ""

    print("\nRules: ")
    print(f"\t{length_valid=}")
    print(f"\t{is_only_alphanumeric=}")
    print(f"\t{contains_vowel=}")
    print(f"\t{contains_consonant=}")
    print(f"\"{word}\" is{_not} valid!")


if __name__ == "__main__":
    test("234Adas")
    test("b3")
    test("a3$e")
    test("Ii4")

# Explanation in this video
# url: https://www.instagram.com/reel/C7hF_0YqXSA/?igsh=MW9neTVnemRjMDU4dA%3D%3D

class Solution:
    def numJewelsInStones(self, jewels: str, stones: str) -> int:
        jewels = set(jewels)
        num_jewels: int = 0

        for stone in stones:
            if stone in jewels:
                num_jewels += 1

        return num_jewels


def test(jewels: str, stones: str) -> None:
    result: int = Solution().numJewelsInStones(jewels, stones)
    print(f"{result} jewels: '{jewels}' in stones: '{stones}'")


if __name__ == "__main__":
    test(jewels="aA", stones="aAAbbbb")
    test(jewels="z", stones="ZZ")

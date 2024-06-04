from typing import List


class Solution:
    def asteroidsDestroyed(self, mass: int, asteroids: List[int]) -> bool:
        asteroids.sort()

        for asteroid in asteroids:
            if mass >= asteroid:
                mass += asteroid
            else:
                return False
        return True


def test(mass: int, asteroids: List[int]) -> None:
    result: bool = Solution().asteroidsDestroyed(mass, asteroids)
    if result:
        print(f"All asteroids: {asteroids} collided with planet: {mass}")
    else:
        print(f"Not all asteroids: {asteroids} collided with planet: {mass}")


if __name__ == "__main__":
    test(mass=10, asteroids=[3, 9, 19, 5, 21])
    test(mass=5, asteroids=[4, 9, 23, 4])
    test(mass=70, asteroids=[100, 90, 100, 10, 15])

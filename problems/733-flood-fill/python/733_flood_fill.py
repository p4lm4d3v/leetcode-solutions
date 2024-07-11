from typing import List


class Solution:
    def floodFill(
        self, image: List[List[int]], sr: int, sc: int, color: int
    ) -> List[List[int]]:
        m, n = len(image), len(image[0])
        target: int = image[sr][sc]

        if color == target:
            return image

        def dfs(i: int, j: int) -> None:
            if i < 0 or i >= m or j < 0 or j >= n or image[i][j] != target:
                return

            image[i][j] = color

            dfs(i - 1, j)
            dfs(i + 1, j)
            dfs(i, j - 1)
            dfs(i, j + 1)

        dfs(sr, sc)
        return image


def floodFillTest(
    idx: int,
    image: List[List[int]],
    sr: int,
    sc: int,
    color: int,
    solution: List[List[int]],
) -> None:
    result: int = Solution().floodFill(image, sr, sc, color)
    if str(result) == str(solution):
        print(f" + 'Test {idx}' passed!")
    else:
        print(f" + 'Test {idx}' failed!")


if __name__ == "__main__":
    floodFillTest(
        1, [[1, 1, 1], [1, 1, 0], [1, 0, 1]], 1, 1, 2, [[2, 2, 2], [2, 2, 0], [2, 0, 1]]
    )
    floodFillTest(2, [[0, 0, 0], [0, 0, 0]], 0, 0, 0, [[0, 0, 0], [0, 0, 0]])

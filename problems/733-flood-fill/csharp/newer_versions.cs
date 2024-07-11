public static class Program
{
  public static void FloodFillTest(int idx, int[][] image, int sr, int sc, int color, int[][] solution)
  {
    int[][] result = new Solution().FloodFill(image, sr, sc, color);
    if (Solution.ToString(result) == Solution.ToString(solution)) { Console.WriteLine(" + Test {0} passed!", idx); }
    else { Console.WriteLine(" - Test {0} failed!", idx); }
  }
  public static void Main(string[] args)
  {
    FloodFillTest(
      1,
      [[1, 1, 1], [1, 1, 0], [1, 0, 1]],
      1,
      1,
      2,
      [[2, 2, 2], [2, 2, 0], [2, 0, 1]]
    );

    FloodFillTest(
      2,
      [[0, 0, 0], [0, 0, 0]],
      0,
      0,
      0,
      [[0, 0, 0], [0, 0, 0]]
    );
  }
}

public class Solution
{
  public int[][] FloodFill(int[][] image, int sr, int sc, int color)
  {
    int m = image.Length;
    int n = image.First().Length;
    int target = image[sr][sc];

    if (color == target)
      return image;

    void DFS(int i, int j)
    {
      // if i or j is out of bound or
      // the field is already colored return
      if (i < 0 || i >= m ||
          j < 0 || j >= n ||
          image[i][j] != target) return;

      // Color the field at i, j
      image[i][j] = color;

      // Up
      DFS(i - 1, j);
      // Down
      DFS(i + 1, j);
      // Left
      DFS(i, j - 1);
      // Right
      DFS(i, j + 1);
    }

    // Start at the source row and source column
    DFS(sr, sc);
    return image;
  }
  public static string ToString(int[][] image)
  {
    string str = string.Empty;

    str += "[";
    foreach (int[] row in image)
    {
      str += "[";
      str += string.Join(",", row);
      str += "]";
    }
    str += "]";

    return str;
  }
}
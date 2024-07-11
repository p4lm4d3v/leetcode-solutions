using System.Collections.Generic;
using System.Linq;
using System;

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
      new int[][] {
        new int[] {1, 1, 1},
        new int[] {1, 1, 0},
        new int[] {1, 0, 1},
      },
      1,
      1,
      2,
      new int[][] {
        new int[] {2, 2, 2},
        new int[] {2, 2, 0},
        new int[] {2, 0, 1}
      }
    );

    FloodFillTest(
      2,
      new int[][] {
            new int[] {0, 0, 0},
            new int[] {0, 0, 0},
      },
      0,
      0,
      0,
      new int[][] {
        new int[] {0, 0, 0},
        new int[] {0, 0, 0},
      }
    );
  }
}

public class Solution
{
  public int[][] FloodFill(int[][] image, int sr, int sc, int color)
  {
    int target = image[sr][sc];
    if (color == target)
      return image;

    // Start at the source row and source column
    Solution.DFS(sr, sc, target, color, image);
    return image;
  }
  static void DFS(int i, int j, int target, int color, int[][] image)
  {
    // Get the size of the image
    int m = image.Length;
    int n = image.First().Length;

    // if i or j is out of bound or
    // the field is already colored return
    if (i < 0 || i >= m ||
        j < 0 || j >= n ||
        image[i][j] != target) return;

    // Color the field at i, j
    image[i][j] = color;

    // Up
    DFS(i - 1, j, target, color, image);
    // Down
    DFS(i + 1, j, target, color, image);
    // Left
    DFS(i, j - 1, target, color, image);
    // Right
    DFS(i, j + 1, target, color, image);
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
using System.Linq;
using System;

public static class Program
{
  public static void Main(string[] args)
  {
    Solution s = new Solution();
    Console.WriteLine(string.Join(", ", s.ProductExceptSelf(new int[] { 1, 2, 3, 4 })));
    Console.WriteLine(string.Join(", ", s.ProductExceptSelf(new int[] { -1, 1, 0, -3, 3 })));
  }
}

public class Solution
{
  public int[] ProductExceptSelf(int[] nums)
  {
    int[] result = new int[nums.Length];

    for (int i = 0; i < nums.Length; i++)
    {
      int left = 1;
      for (int l = 0; l < i; l++)
        left *= nums[l];

      int right = 1;
      for (int r = i + 1; r < nums.Length; r++)
        right *= nums[r];

      result[i] = left * right;
    }

    return result;
  }
}
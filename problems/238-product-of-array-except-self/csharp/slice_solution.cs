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

    for (int i = 0, j = 1; i < nums.Length; i++, j++)
    {
      int left = ArrayProduct(nums[..i]);
      int right = ArrayProduct(nums[j..]);

      result[i - 1] = left * right;
    }

    return result;
  }
  int ArrayProduct(int[] nums)
  {
    int p = 1;
    for (int i = 0; i < nums.Length; i++)
      p *= nums[i];
    return p;
  }
}
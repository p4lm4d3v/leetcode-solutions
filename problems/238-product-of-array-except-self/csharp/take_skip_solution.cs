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

    for (int i = 1; i <= nums.Length; i++)
    {
      int[] leftSide = nums.Take(i - 1).ToArray();
      int left = ArrayProduct(leftSide);

      int[] rightSide = nums.Skip(i).ToArray();
      int right = ArrayProduct(rightSide);
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
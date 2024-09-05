using System;

public static class Program
{
  public static void MaximumGapTest(int[] nums, int expected)
  {
    int result = new Solution().MaximumGap(nums);
    if (result == expected)
      Console.WriteLine(" + Test 'Convert({0}) == {1}' passed!", string.Join("", nums), expected);
    else
      Console.WriteLine(" - Test 'Convert({0}) == {1}' failed with value {2}!", string.Join("", nums), expected, result);
  }
  public static void Main(string[] args)
  {
    MaximumGapTest([3, 6, 9, 1], 3);
    MaximumGapTest([10], 0);
  }
}

public class Solution
{
  public int MaximumGap(int[] nums)
  {
    if (nums.Length < 2) return 0;

    Array.Sort(nums);

    int max = 0;
    for (int i = nums.Length - 1; i > 0; i--)
      max = Math.Max(max, nums[i] - nums[i - 1]);

    return max;
  }
}
using System.Collections.Generic;
using System.Linq;
using System;

public static class Program
{
  public static void MissingNumberTest(int[] nums, int solution)
  {
    int result = new Solution().MissingNumber(nums);
    if (result == solution) { Console.WriteLine(" + Test 'MN([{0}]) == {1}' passed!", string.Join(",", nums), solution); }
    else { Console.WriteLine(" - Test 'MN([{0}]) == {1}' failed with value {2}!", string.Join(",", nums), solution, result); }
  }
  public static void Main(string[] args)
  {
    MissingNumberTest(new int[] { 3, 0, 1 }, 2);
    MissingNumberTest(new int[] { 0, 1 }, 2);
    MissingNumberTest(new int[] { 9, 6, 4, 2, 3, 5, 7, 0, 1 }, 8);
  }
}

public class Solution
{
  public int MissingNumber(int[] nums)
  {
    int sum = nums.ToList().Sum();
    int expected = nums.Length * (nums.Length + 1) / 2;
    return expected - sum;
  }
}
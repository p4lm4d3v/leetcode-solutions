using System.Collections.Generic;
using System;

public static class Program
{
  public static void FindMaxConsecutiveOnesTest(int[] nums, int expected)
  {
    int result = new Solution().FindMaxConsecutiveOnes(nums);
    if (result == expected) { Console.WriteLine(" + Test 'FindMaxConsecutiveOnes([{0}]) == {1}' passed!", string.Join(",", nums), expected); }
    else { Console.WriteLine(" - Test 'FindMaxConsecutiveOnes([{0}]) == {1}' failed with value {2}!", string.Join(",", nums), expected, result); }
  }

  public static void Main(string[] args)
  {
    FindMaxConsecutiveOnesTest(new int[] {1,1,0,1,1,1}, 3);
    FindMaxConsecutiveOnesTest(new int[] {1,0,1,1,0,1}, 2);  
  }
}

public class Solution {
  public int FindMaxConsecutiveOnes(int[] input) {
    int max = int.MinValue;
    int counter = 0;

    int[] nums = new int[input.Length + 1];
    for (int i = 0; i < input.Length; i++)
      nums[i] = input[i];

    nums[input.Length] = 0;

    foreach (int n in nums) {
      if (n == 1)
        counter++;
      else if (n == 0) {
        if (counter > max)
          max = counter;
        counter = 0;
      }
    }

    return max;
  }
}


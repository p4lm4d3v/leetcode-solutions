using System.Collections.Generic;
using System.Linq;
using System;

public static class Program
{
    public static void LongestConsecutiveTest(int[] nums, int expected)
    {
        int result = new Solution().LongestConsecutive(nums);
        if (result == expected) { Console.WriteLine(" + Test 'LongestConsecutive({0}) == {1}' passed!", string.Join("", nums), expected); }
        else { Console.WriteLine(" - Test 'LongestConsecutive({0}) == {1}' failed with value {2}!", string.Join("", nums), expected, result); }
    }
    public static void Main(string[] args)
    {
        LongestConsecutiveTest(new int[] { 100, 4, 200, 1, 3, 2 }, 4);
        LongestConsecutiveTest(new int[] { 0, 3, 7, 2, 5, 8, 4, 6, 0, 1 }, 9);
    }
}


public class Solution
{
    public int LongestConsecutive(int[] nums)
    {
        // First we turn the [nums] array into a hash set for O(n) lookup
        HashSet<int> set = nums.ToHashSet();

        // Then we define the [longest] variable that represents the longest sequence
        int longest = 0;

        // We iterate over each item from the [set]
        foreach (int n in set)
            // If the [set] doesn't contain the previous number we continue,
            // meaning that the number [n] is a start of the sequence 
            if (!set.Contains(n - 1))
            {
                // We define the [length] for the current sequence
                int length = 1;

                // While the [set] contains the next number in the sequence
                // we extend the sequence [length]
                while (set.Contains(n + length))
                    // By increasing the [length] by +1
                    length++;

                // The while loop ended, meaning that we found the lenght of the sequence 

                // We set the [longest] to itself or the length by checking the larger number 
                longest = Math.Max(length, longest);
            }

        // We end the function by returning [longest] 
        return longest;
    }
}
using System.Collections.Generic;
using System.Linq;
using System;

public static class Program
{
  public static void FrequencySortTest(int[] nums, int[] expected)
  {
    int[] result = new Solution().FrequencySort(nums);
    if (string.Join("", result) == string.Join("", expected))
      Console.WriteLine(" + Test 'FS([{0}]) == [{1}]' passed!", string.Join(",", nums), string.Join(",", expected));
    else
      Console.WriteLine(" + Test 'FS([{0}]) == [{1}]' failed with value [{2}]!", string.Join(",", nums), string.Join(",", expected), string.Join(",", result));
  }
  public static void Main(string[] args)
  {
    FrequencySortTest(new int[] { 1, 1, 2, 2, 2, 3 }, new int[] { 3, 1, 1, 2, 2, 2 });
    FrequencySortTest(new int[] { 2, 3, 1, 3, 2 }, new int[] { 1, 3, 3, 2, 2 });
    FrequencySortTest(new int[] { -1, 1, -6, 4, 5, -6, 1, 4, 1 }, new int[] { 5, -1, 4, 4, -6, -6, 1, 1, 1 });
  }
}
public class Solution
{
  public int[] FrequencySort(int[] nums)
  {
    Dictionary<int, int> freq = new Dictionary<int, int>();
    foreach (int n in nums)
      if (freq.ContainsKey(n))
        freq[n] += 1;
      else
        freq[n] = 1;

    List<KeyValuePair<int, int>> freqList = freq.ToList();
    freqList.Sort((a, b) =>
    {
      if (a.Value == b.Value)
        return b.Key.CompareTo(a.Key);
      return a.Value.CompareTo(b.Value);
    });

    List<int> result = new List<int>();
    foreach (KeyValuePair<int, int> f in freqList)
      for (int i = 0; i < f.Value; i++)
        result.Add(f.Key);

    return result.ToArray();
  }
}
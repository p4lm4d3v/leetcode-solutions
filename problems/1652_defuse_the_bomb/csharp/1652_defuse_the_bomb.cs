using System.Linq;
using System;

public static class Program
{
  public static void Main(string[] args)
  {
    Solution s = new Solution();
    Console.WriteLine(string.Join(", ", s.Decrypt(new int[] { 5, 7, 1, 4 }, 3)));
    Console.WriteLine(string.Join(", ", s.Decrypt(new int[] { 1, 2, 3, 4 }, 0)));
    Console.WriteLine(string.Join(", ", s.Decrypt(new int[] { 2, 4, 9, 3 }, -2)));
  }
}

public class Solution
{
  int LoopIdx(int idx, int len) { return (idx + len) % len; }
  public int[] Decrypt(int[] code, int k)
  {
    int len = code.Length;
    int[] result = code.Select(x => 0).ToArray();

    if (k == 0)
      return result;

    if (k < 0)
      Array.Reverse(code);

    for (int i = 0; i < len; i++)
      result[i] = SumK(code, i, Math.Abs(k));

    if (k < 0)
      Array.Reverse(result);

    return result;
  }
  int SumK(int[] code, int i, int k)
  {
    int result = 0;

    for (int j = 0; j < k; j++)
      result += code[LoopIdx(i + 1 + j, code.Length)];

    return result;
  }
}
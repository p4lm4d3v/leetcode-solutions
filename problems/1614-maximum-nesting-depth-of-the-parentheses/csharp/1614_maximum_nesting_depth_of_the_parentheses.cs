using System;

public static class Program
{
  public static void Main(string[] args)
  {
    Solution s = new Solution();
    Console.WriteLine(s.MaxDepth("(1+(2*3)+((8)/4))+1"));
    Console.WriteLine(s.MaxDepth("(1)+((2))+(((3)))"));
    Console.WriteLine(s.MaxDepth("()(())((()()))"));
  }
}

public class Solution
{
  public int MaxDepth(string s)
  {
    int max = 0;
    int depth = 0;
    foreach (char c in s)
    {
      switch (c)
      {
        case '(': depth++; break;
        case ')': depth--; break;
      }
      if (depth > max) max = depth;
    }
    return max;
  }
}
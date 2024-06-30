using System.Collections.Generic;
using System;

public static class Program
{
  public static void Main(string[] args)
  {
    Solution s = new Solution();
    Console.WriteLine(s.IsIsomorphic("egg", "add"));
    Console.WriteLine(s.IsIsomorphic("foo", "bar"));
    Console.WriteLine(s.IsIsomorphic("paper", "title"));
  }
}

public class Solution
{
  public bool IsIsomorphic(string s, string t)
  {
    if (s.Length != t.Length) return false;
    if (s.Equals(t)) return true;
    return ToNumeric(s).Equals(ToNumeric(t));
  }
  public string ToNumeric(string str)
  {
    Dictionary<char, int> dict = new Dictionary<char, int>();
    int n = 1;
    string numeric = string.Empty;
    foreach (char c in str)
      if (dict.ContainsKey(c))
        numeric += dict[c];
      else
      {
        dict.Add(c, n);
        numeric += n.ToString();
        n++;
      }
    return numeric;
  }
}
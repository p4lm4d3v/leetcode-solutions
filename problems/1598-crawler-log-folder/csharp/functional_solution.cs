using System.Collections.Generic;
using System.Linq;
using System;

public static class Program
{
  public static void MinOperationsTest(string[] logs, int solution)
  {
    int result = new Solution().MinOperations(logs);
    if (solution == result) { Console.WriteLine(" + 'Test MO([{0}]) == {1}' passed!", string.Join(",", logs), solution); }
    else { Console.WriteLine(" - 'Test MO([{0}]) == {1}' failed with value {3}", string.Join(",", logs), solution, result); }
  }
  public static void Main(string[] args)
  {
    MinOperationsTest(new string[] { "d1/", "d2/", "../", "d21/", "./" }, 2);
    MinOperationsTest(new string[] { "d1/", "d2/", "./", "d3/", "../", "d31/" }, 3);
    MinOperationsTest(new string[] { "d1/", "../", "../", "../" }, 0);
  }
}

public class Solution
{
  public int MinOperations(string[] logs)
  {
    List<int> vals = logs.ToList().Select(log =>
    {
      if (log == "../") return -1;
      else if (log != "./") return 1;
      else return 0;
    }).ToList();

    return Math.Max(0, vals.Sum());
  }
}
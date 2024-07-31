using System.Collections.Generic;
using System;

public static class Program
{
  public static void ConvertTest(string s, int numRows, string expected)
  {
    string result = new Solution().Convert(s, numRows).Trim();
    if (result.ToLower() == expected.ToLower()) { Console.WriteLine(" + Test 'Convert({0}, {1}) == {2}' passed!", s, numRows, expected); }
    else { Console.WriteLine(" - Test 'Convert({0}, {1}) == {2}' failed with value {3}!", s, numRows, expected, result); }
  }
  public static void Main(string[] args)
  {
    ConvertTest("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR");
    ConvertTest("PAYPALISHIRING", 4, "PINALSIGYAHRPI");
    ConvertTest("A", 1, "A");
    ConvertTest("ABC", 2, "ACB");
    ConvertTest("AB", 1, "AB");
  }
}

public class Solution
{
  public string Convert(string s, int rows)
  {
    if (rows == 1)
      return s;

    int n = s.Length;
    List<List<char>> matrix = new List<List<char>>();

    for (int i = 0; i < Math.Min(rows, n); i++)
      matrix.Add(new List<char>());

    bool zigzag = false;
    int curRow = 0;

    foreach (char c in s)
    {
      matrix[curRow].Add(c);

      if (curRow == 0 || curRow == rows - 1)
        zigzag = !zigzag;

      curRow += zigzag ? 1 : -1;
    }

    List<char> result = new List<char>();

    foreach (List<char> row in matrix)
      result.AddRange(row);

    return string.Join("", result);
  }
}

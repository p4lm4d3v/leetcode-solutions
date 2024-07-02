using System;

public static class Program
{
  public static void Main(string[] args)
  {
    Solution s = new Solution();
    Console.WriteLine(s.NumberOfSteps(14));
    Console.WriteLine(s.NumberOfSteps(8));
    Console.WriteLine(s.NumberOfSteps(123));
  }
}

public class Solution
{
  public int NumberOfSteps(int number)
  {
    int counter = 0;
    while (number != 0)
    {
      if (number % 2 == 0) number /= 2;
      else number -= 1;
      counter++;
    }
    return counter;
  }
}
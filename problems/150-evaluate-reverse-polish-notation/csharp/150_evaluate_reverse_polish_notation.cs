using System.Collections.Generic;
using System;

public static class Program
{
  public static void Main(string[] args)
  {
    Solution s = new Solution();
    Console.WriteLine(s.EvalRPN(new string[] { "2", "1", "+", "3", "*" }));
    Console.WriteLine(s.EvalRPN(new string[] { "4", "13", "5", "/", "+" }));
    Console.WriteLine(s.EvalRPN(new string[] { "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+" }));
  }
}

public class Solution
{
  public int EvalRPN(string[] tokens)
  {
    Stack<int> stack = new Stack<int>();
    HashSet<string> operators = new HashSet<string> { "+", "-", "*", "/" };

    foreach (string token in tokens)
    {
      if (!operators.Contains(token))
        stack.Push(int.Parse(token));
      else
      {
        int b = stack.Pop();
        int a = stack.Pop();
        switch (token)
        {
          case "+": stack.Push(a + b); break;
          case "-": stack.Push(a - b); break;
          case "*": stack.Push(a * b); break;
          case "/": stack.Push(a / b); break;
        }
      }
    }

    return stack.Pop();
  }
}
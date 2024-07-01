using System.Collections.Generic;
using System.Linq;
using System;

public static class Program
{
  public static void Main(string[] args)
  {
    Solution s = new Solution();
    Console.WriteLine(s.CountStudents(new int[] { 1, 1, 0, 0 }, new int[] { 0, 1, 0, 1 }));
    Console.WriteLine(s.CountStudents(new int[] { 1, 1, 1, 0, 0, 1 }, new int[] { 1, 0, 0, 0, 1, 1 }));
  }
}

public class Solution
{
  public int CountStudents(int[] studentsArr, int[] sandwichesArr)
  {
    Queue<int> students = new Queue<int>();
    foreach (int student in studentsArr)
      students.Enqueue(student);

    Stack<int> sandwiches = new Stack<int>();
    foreach (int sandwich in sandwichesArr.Reverse())
      sandwiches.Push(sandwich);

    int count = 0;
    while (students.Count != 0 && sandwiches.Count != 0)
    {
      int student = students.Peek();
      int sandwich = sandwiches.Peek();

      string before = string.Join("", students);

      if (student != sandwich)
        students.Enqueue(students.Dequeue());
      else
      {
        students.Dequeue();
        sandwiches.Pop();
      }

      string after = string.Join("", students);

      if (before == after)
      {
        count = before.Length;
        break;
      }
    }

    return count;
  }
}
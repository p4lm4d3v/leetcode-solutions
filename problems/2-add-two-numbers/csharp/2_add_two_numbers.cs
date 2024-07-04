using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System;

public static class Program
{
  public static void LinkedListFromValuesTest(List<int> a, ListNode b)
  {
    ListNode node = new Solution().LinkedListFromValues(a);
    if (node.ToString() == b.ToString()) { Console.WriteLine(" + Test '{0} == {1}' passed!", node, b); }
    else { Console.WriteLine(" - Test '{0} == {1}' failed!", node, b); }

  }
  public static void AddTwoNumberTest(List<int> a, List<int> b, List<int> c)
  {
    ListNode node1 = new Solution().LinkedListFromValues(a);
    ListNode node2 = new Solution().LinkedListFromValues(b);
    ListNode solution = new Solution().LinkedListFromValues(c);

    ListNode result = new Solution().AddTwoNumbers(node1, node2);

    if (result.ToString() == solution.ToString()) { Console.WriteLine(" + Test '{0} + {1} == {2}' passed!", node1, node2, solution); }
    else { Console.WriteLine(" - Test '{0} + {1} == {2} with value {3}' failed!", node1, node2, solution, result); }
  }
  public static void Main(string[] args)
  {
    LinkedListFromValuesTest(
        new List<int> { 2, 4, 3 },
        new ListNode(2, new ListNode(4, new ListNode(3)))
    );
    LinkedListFromValuesTest(
        new List<int> { 5, 6, 4 },
        new ListNode(5, new ListNode(6, new ListNode(4)))
    );
    LinkedListFromValuesTest(
        new List<int> { 7, 0, 8 },
        new ListNode(7, new ListNode(0, new ListNode(8)))
    );
    AddTwoNumberTest(
        new List<int> { 2, 4, 3 }, new List<int> { 5, 6, 4 },
        new List<int> { 7, 0, 8 }
    );

    Console.WriteLine();

    LinkedListFromValuesTest(
        new List<int> { 9, 9, 9, 9, 9, 9, 9 },
        new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9)))))))
    );
    LinkedListFromValuesTest(
        new List<int> { 9, 9, 9, 9 },
        new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9))))
    );
    LinkedListFromValuesTest(
        new List<int> { 8, 9, 9, 9, 0, 0, 0, 1 },
        new ListNode(8, new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(0, new ListNode(0, new ListNode(0, new ListNode(1))))))))
    );
    AddTwoNumberTest(
        new List<int> { 9, 9, 9, 9, 9, 9, 9 }, new List<int> { 9, 9, 9, 9 },
        new List<int> { 8, 9, 9, 9, 0, 0, 0, 1 }
    );
  }
}

public class ListNode
{
  public int val;
  public ListNode next;

  public ListNode(int val = 0, ListNode next = null)
  {
    this.val = val;
    this.next = next;
  }

  public override String ToString()
  {
    string str = String.Empty;
    ListNode current = this;

    while (current != null)
    {
      str += current.val;
      current = current.next;
    }

    return string.Join("", str.Reverse());
  }
}
public class Solution
{
  public ListNode AddTwoNumbers(ListNode l1, ListNode l2)
  {
    ListNode curr1 = l1;
    ListNode curr2 = l2;

    int carry = 0;
    List<int> values = new List<int>();

    while (curr1 != null || curr2 != null)
    {
      int val1 = curr1 == null ? 0 : curr1.val;
      int val2 = curr2 == null ? 0 : curr2.val;
      int value = val1 + val2 + carry;

      carry = value / 10;
      value = value % 10;
      values.Add(value);

      curr1 = curr1 == null ? curr1 : curr1.next;
      curr2 = curr2 == null ? curr2 : curr2.next;

      if (curr1 == null && curr2 == null && carry != 0)
        values.Add(carry);
    }

    ListNode result = new ListNode(values[0]);
    ListNode curr = result;

    for (int i = 1; i < values.Count; i++)
    {
      curr.next = new ListNode(values[i]);
      curr = curr.next;
    }

    return result;
  }
  public ListNode LinkedListFromValues(List<int> values)
  {
    ListNode result = new ListNode(values[0]);
    ListNode current = result;

    for (int i = 1; i < values.Count; i++)
    {
      current.next = new ListNode(values[i]);
      current = current.next;
    }

    return result;
  }
}
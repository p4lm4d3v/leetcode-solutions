using System;

public static class Program
{
    public static void Main(string[] args)
    {
        Solution s = new Solution();
        Console.WriteLine(s.LengthOfLastWord("Hello World"));
        Console.WriteLine(s.LengthOfLastWord("   fly me   to   the moon  "));
        Console.WriteLine(s.LengthOfLastWord("luffy is still joyboy"));
    }
}

public class Solution
{
    public int LengthOfLastWord(string s)
    {
        s = s.Trim();
        string parsed = string.Empty;
        for (int i = s.Length - 1; i >= 0; i--)
        {
            if (char.IsWhiteSpace(s[i])) break;
            else parsed += s[i];
        }
        return parsed.Length;
    }
}
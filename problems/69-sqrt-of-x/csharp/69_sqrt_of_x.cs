using System.Numerics;
using System;

public static class Program 
{
	public static void Main(string[] args) 
	{
		Solution s = new Solution();
		Console.WriteLine(s.MySqrt(4));
		Console.WriteLine(s.MySqrt(8));
		Console.WriteLine(s.MySqrt(2147395599));
		Console.WriteLine(s.MySqrt(2147395600));
	}
}

public class Solution
{
    public int MySqrt(int x)
    {
        BigInteger i = new BigInteger(0);

        while (i * i <= x)
        {
            if ((i + 1) * (i + 1) > x)
                break;
            i++;
        }

        return (int)i;
    }
}
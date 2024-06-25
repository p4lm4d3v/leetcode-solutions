public class Solution
{
    public int Fib(int n, Dictionary<int, int> cache = null)
    {
        if (n == 0) return 0;
        if (n == 1) return 1;

        if (cache == null) cache = new Dictionary<int, int>();
        if (cache.ContainsKey(n)) return cache[n];

        int twoBack = Fib(n - 2);
        int oneBack = Fib(n - 1);
        cache.Add(n, twoBack + oneBack);
        return twoBack + oneBack;
    }
}
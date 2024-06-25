public class Solution
{
    public int ClimbStairs(int n, Dictionary<int, int> cache = null)
    {
        if (n == 1) return 1;
        if (n == 2) return 2;

        if (cache == null) cache = new Dictionary<int, int>();
        if (cache.ContainsKey(n)) return cache[n];

        int twoBack = this.ClimbStairs(n - 2, cache);
        int oneBack = this.ClimbStairs(n - 1, cache);
        cache.Add(n, twoBack + oneBack);
        return twoBack + oneBack;
    }
}
public class Solution
{
    public int Fib(int n)
    {
        if (n == 0) return 0;
        if (n == 1) return 1;

        List<int> vec = new List<int> { 1, 2 };
        int idx = 0;

        for (int i = 2; i < n; i++)
        {
            int twoBack = vec[i - 2];
            int oneBack = vec[i - 1];
            vec.Add(twoBack + oneBack);
            idx = i - 1;
        }

        return vec[idx];
    }
}
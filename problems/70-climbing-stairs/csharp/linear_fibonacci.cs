public class Solution
{
    public int ClimbStairs(int n)
    {
        if (n == 1) return 1;
        if (n == 2) return 2;

        List<int> vec = new List<int> {1, 2};
        int idx = 0;

        for(int i = 2; i < n; i++) {
            int twoBack = vec[i - 2];
            int oneBack = vec[i - 1];
            vec.Add(twoBack + oneBack);
            idx = i;
        }
        return vec[idx];
    }
}
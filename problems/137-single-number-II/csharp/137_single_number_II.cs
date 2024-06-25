public class Solution {
    public int SingleNumber(int[] nums) {
        Dictionary<int, int> dict = new Dictionary<int, int>();

        foreach (int n in nums)
            if (dict.ContainsKey(n))
                dict[n]++;
            else
                dict.Add(n, 1);

        foreach (KeyValuePair<int, int> kvp in dict) 
            if (kvp.Value == 1)
                return kvp.Key;
            
        return -1;
    }
}
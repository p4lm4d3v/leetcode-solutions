public class Solution {
    public int[] SingleNumber(int[] nums) {
        Dictionary<int, int> dict = new Dictionary<int, int>();

        foreach (int n in nums)
            if (dict.ContainsKey(n))
                dict[n]++;
            else
                dict.Add(n, 1);

        List<int> singles = new List<int>();
        foreach (KeyValuePair<int, int> kvp in dict) 
            if (kvp.Value == 1)
                singles.Add(kvp.Key);
            
        return singles.ToArray();
    }
}

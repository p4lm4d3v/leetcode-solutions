public class Solution {
    public int SingleNumber(int[] nums) {
        Dictionary<int, int> dict = new Dictionary<int, int>();

        for (int i = 0; i < nums.Length; i++)
            if (dict.ContainsKey(nums[i]))
                dict[nums[i]]++;
            else
                dict.Add(nums[i], 1);

        foreach (KeyValuePair<int, int> kvp in dict)
            if (kvp.Value == 1)
                return kvp.Key;
        return -1;
    }
}
public class Solution {
    public int[] SortedSquares(int[] nums)
    {
        int[] squares = nums.ToList().Select(i => i * i).ToArray();
        Array.Sort(squares);
        return squares;
    }
}
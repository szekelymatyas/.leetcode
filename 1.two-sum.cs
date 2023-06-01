/*
 * @lc app=leetcode id=1 lang=csharp
 *
 * [1] Two Sum
 */

// @lc code=start
public class Solution {
    public int[] TwoSum(int[] nums, int target) {
        var dic = new Dictionary<int, int>();
        var lenght = nums.Length;
        for (int i = 0; i < lenght; i++)
        {
            if(dic.TryGetValue(target - nums[i], out int value))
                return new int[]{value, i};
            else
                dic.TryAdd(nums[i], i);

        }
        return new int[0];
    }
}
// @lc code=end


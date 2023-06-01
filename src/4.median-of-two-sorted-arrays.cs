/*
 * @lc app=leetcode id=4 lang=csharp
 *
 * [4] Median of Two Sorted Arrays
 */

// @lc code=start
public class Solution {
    public double FindMedianSortedArrays(int[] nums1, int[] nums2) {
        var count = nums1.Length + nums2.Length;
        if(count % 2 == 0)
        {
            var pos1 = 0;
            var pos2 = 0;
            int i = 0;
            var goal = (count / 2) - 1;
            while (i < goal)
            {
                if(nums1.Length <= pos1)
                {
                    pos2++;
                    i++;
                    continue;
                }
                if (nums2.Length <= pos2)
                {
                    pos1++;
                    i++;
                    continue;
                }

                if (nums1[pos1] < nums2[pos2])
                {
                    pos1++;
                    i++;
                }
                else
                {
                    pos2++;
                    i++;
                }
            }
            var sum = 0;
            var goal2 = (count / 2) + 1;
            while (i < goal2)
            {
                if (nums1.Length <= pos1)
                {
                    sum += nums2[pos2];
                    pos2++;
                    i++;
                    continue;
                }
                if (nums2.Length <= pos2)
                {
                    sum += nums1[pos1];
                    pos1++;
                    i++;
                    continue;
                }

                if (nums1[pos1] < nums2[pos2])
                {
                    sum += nums1[pos1];
                    pos1++;
                    i++;
                }
                else
                {
                    sum += nums2[pos2];
                    pos2++;
                    i++;
                }
            }
            return sum / 2.0;
        }
        else
        {
            var pos1 = 0;
            var pos2 = 0;
            int i = 0;
            var goal = (count / 2);
            while (i < goal)
            {
                if (nums1.Length <= pos1)
                {
                    pos2++;
                    i++;
                    continue;
                }
                if (nums2.Length <= pos2)
                {
                    pos1++;
                    i++;
                    continue;
                }

                if (nums1[pos1] < nums2[pos2])
                {
                    pos1++;
                    i++;
                }
                else
                {
                    pos2++;
                    i++;
                }
            }
            if (nums1.Length <= pos1)
            {
                return nums2[pos2];
            }
            if (nums2.Length <= pos2)
            {
                return nums1[pos1];
            }

            if (nums1[pos1] < nums2[pos2])
            {
                return nums1[pos1];
            }
            else
            {
                return nums2[pos2];
            }
        }
    }
}
// @lc code=end


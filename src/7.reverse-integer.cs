/*
 * @lc app=leetcode id=7 lang=csharp
 *
 * [7] Reverse Integer
 */

// @lc code=start
public class Solution
{
    public int Reverse(int x)
    {
        if (x < 0)
        {
            var s = x.ToString().AsSpan();
            Span<char> s2 = stackalloc char[s.Length];
            s2[0] = '-';
            for (int i = 1; i < s.Length; i++)
            {
                s2[^(i)] = s[i];
            }
            return int.TryParse(s2, out int res) ? res : 0;
        }
        else
        {
            var s = x.ToString().AsSpan();
            Span<char> s2 = stackalloc char[s.Length];
            for (int i = 0; i < s.Length; i++)
            {
                s2[^(i + 1)] = s[i];
            }
            return int.TryParse(s2, out int res) ? res : 0;
        }

    }
}
// @lc code=end


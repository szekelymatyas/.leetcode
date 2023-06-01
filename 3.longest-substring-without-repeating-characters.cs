/*
 * @lc app=leetcode id=3 lang=csharp
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
public class Solution {
    public int LengthOfLongestSubstring(string s) {
        var span = s.AsSpan();
            int pos = 0;
            int lenght = 0;
            int start = 0;
            Dictionary<char, int> chars = new();
            while (pos < span.Length)
            {
                char ch = span[pos];
                if (chars.TryGetValue(ch, out int prevPos))
                {
                    foreach (char ch2 in span[start..(prevPos+1)]) {
                        chars.Remove(ch2);
                    }
                    chars.Add(ch, pos);
                    start = prevPos+1;
                }
                else
                {
                    chars.Add(ch, pos);
                }
                lenght = lenght < pos - start + 1? pos - start + 1: lenght;
                pos++;
            }

            return lenght;
    }
}
// @lc code=end


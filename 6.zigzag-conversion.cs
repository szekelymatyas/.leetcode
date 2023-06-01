/*
 * @lc app=leetcode id=6 lang=csharp
 *
 * [6] Zigzag Conversion
 */

// @lc code=start
public class Solution {
    public string Convert(string s, int numRows) {
        if (numRows < 1)
            throw new Exception("Invalid numRows");
        if (numRows == 1)
            return s;
        var period = (numRows - 1) * 2;
        Dictionary<int, int> map = new();
        int i = 0;
        while (i < period)
        {
            if (i < numRows)
            {
                map.Add(i, i);
            }
            else
            {
                map.Add(i, period - i);
            }
            i++;
        }
        Dictionary<int, List<char>> rowChars = new();
        for (int pos = 0; pos < s.Length; pos++)
        {
            var rowNum = map[(pos % period)];
            if (rowChars.TryGetValue(rowNum, out var row))
            {
                row.Add(s[pos]);
            }
            else
            {
                rowChars.Add(rowNum, new List<char>() { s[pos] });
            }
        }
        return string.Join(string.Empty, rowChars.Select(x => new string(x.Value.ToArray())));
    }
}
// @lc code=end


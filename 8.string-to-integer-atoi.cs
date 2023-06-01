/*
 * @lc app=leetcode id=8 lang=csharp
 *
 * [8] String to Integer (atoi)
 */

// @lc code=start
public class Solution
{
    public int MyAtoi(string s)
    {
        var length = s.Length;
        if( length == 0)
            return 0;
        long num = 0;
        int sign = 1;
        int i = 0;
        while(i < length)
        {
            var c = s[i++];
            if(c == ' ')             
                continue;
            if(c == '+') 
                break;
            if(c == '-') {
                sign = -1;
                break;
            }
            var n = c - '0';
            if (n < 0 || n > 9)
                return 0;
            num = n;
            break;
        }
        while (i < length)
        {
            var n = s[i++] - '0';
            if(n < 0 || n > 9)
                break;             
            num = num * 10 + n;        
            if(num > int.MaxValue)
                return sign == 1? int.MaxValue: int.MinValue;
        }
        num *= sign;
        return (int)num;
    }
}
// @lc code=end


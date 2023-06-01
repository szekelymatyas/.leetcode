/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
use std::convert::TryInto;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let s: Vec<u8> =  x.to_string().bytes().collect();
        let mut end = s.len()-1;
        let mut start: usize = 0;
        while start < end {
            if s[end] != s[start] 
            {
                return false
            }
            start += 1;
            end -= 1;
        }
        return true;
    }
}
// @lc code=end


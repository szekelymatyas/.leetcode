/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */



// @lc code=start
use std::convert::TryFrom;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let len: usize = s.len();
        let mut num: i64 = 0;
        let mut sign: i32 = 1;
        let mut c: u8 = 0;
        let mut i: usize = 0;
        let vec: Vec<u8>= s.bytes().collect();
        while (i < len){
            c = vec[i];
            i += 1;
            match c {
                b' ' => { continue; }
                b'+' => { break; } 
                b'-' => {
                    sign = -1;
                    break;
                } 
                n @ b'0'..=b'9' => {
                    num += i64::try_from(n - b'0').unwrap();
                    break;
                }
                _ => { return 0; }
            };
        }
        while (i < len){
            c = vec[i];
            i += 1;
            match c {
                n @ b'0'..=b'9' => {
                    num = num * 10 + i64::try_from(n - b'0').unwrap();
                    if(num > i32::MAX.into()){
                        match (sign) {
                            1 => { return i32::MAX },
                            -1 => { return i32::MIN },
                            _ => { panic!("Unreachable") }
                        };
                    }
                    continue;
                }
                _ => { break; }
            };
        }
        if(sign == -1){
            num *= -1;
        }
        if(num < i32::MIN.into()){
            return i32::MIN;
        }
        return i32::try_from(num).unwrap();
    }
}
// @lc code=end


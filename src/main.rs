
fn main() {
    test();
}

fn test() -> bool{
    let x = 12321;
    if x < 0 {
        return false;
    }
    let s: Vec<u8> =  x.to_string().bytes().collect();
    let mut end = s.len();
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

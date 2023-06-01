
fn main() {
    println!("Hello, world!");
    let s: String = String::from("Hello, world!");
    let s: Vec<u8> = s.bytes().collect();
    let mut num = 0;
    let mut i = 0;
    while (i < s.len())
    {
        let c = s[i];
        match c {
            b' ' => { continue;}
            b'+' => { }
        }
    }
    return num;
}

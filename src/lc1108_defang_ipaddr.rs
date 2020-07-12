impl Solution {
    // A defanged IP address replaces every period "." with "[.]".
    pub fn defang_i_paddr(address: String) -> String {
        address.chars().map(|ch| {
            if ch == '.' {
                "[.]".to_string()
            } else {
                format!("{}", ch)
            }
        }).collect()
    }
}

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::defang_i_paddr("1.1.1.1".to_string()), "1[.]1[.]1[.]1".to_string());

}
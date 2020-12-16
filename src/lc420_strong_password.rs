struct Solution;

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let chars = password.chars().collect::<Vec<char>>();
        let mut change_count = 0;

        let is_len_ge_6 = chars.len() >= 6;
        let is_len_le_20 = chars.len() <= 20;
        let has_lowercase_case = chars.iter().any(|e| e.is_ascii_lowercase());
        let has_uppercase_case = chars.iter().any(|e| e.is_ascii_uppercase());
        let has_digit = chars.iter().any(|e| e.is_digit(10));
        let has_repeated_chars = chars.windows(3).any(|e| (e[0] == e[1]) && (e[1] == e[2]));

        dbg!(is_len_ge_6);
        dbg!(is_len_le_20);
        dbg!(has_lowercase_case);
        dbg!(has_uppercase_case);
        dbg!(has_digit);
        dbg!(has_repeated_chars);

        if ! has_lowercase_case {
            change_count += 1;
        }
        if ! has_lowercase_case {
            change_count += 1;
        }
        if ! has_digit {
            change_count += 1;
        }
        if has_repeated_chars {
            change_count += 1;
        }

        if ! is_len_ge_6 {
            change_count += (6 - password.len())
        } else if ! is_len_le_20 {
            change_count += (password.len() - 20)
        }

        0
    }
}

#[test]
fn test() {
    use Solution;
    Solution::strong_password_checker("hello0aaa".to_string());
}

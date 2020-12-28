struct Solution;

impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        let value = n.parse::<u32>().unwrap();

        let digits: Vec<u32> = n.chars().map(|ch| ch as u32 - '0' as u32).collect();
    
        let prefix_len = ((n.len() - 1) / 2) as u32;
        //dbg!(prefix_len);
        let num_perms = 2i32.pow(prefix_len);
        //dbg!(num_perms);
    
        let middle = digits[prefix_len as usize] * 10u32.pow(prefix_len);
        //dbg!(middle);
    
        let mut closest = value;
        let mut closest_diff = 0u32;
    
        for i in 0..num_perms {
            let mut gen = middle;
            //println!("\n--- i = {} ---", i);
            for j in 0..prefix_len {
                let from_left = (i & (1 << j) != 0);
                //println!("j = {}, k = {}", j, from_left);
    
                let digit = if from_left {
                    digits[j as usize]
                } else {
                    digits[digits.len() - 1 - j as usize]
                };
                //dbg!(digit);
                gen += (digit * 10u32.pow(2 * prefix_len - j));
                gen += (digit * 10u32.pow(j));
            }
            let diff = (gen as i32 - value as i32).abs() as u32;
            if i == 0 || diff < closest_diff || (diff == closest_diff && gen < closest) {
                if gen != value {
                    closest_diff = diff;
                    closest = gen;
                }
            }
            //dbg!(gen);
        }
        closest.to_string()
    }
}

#[test]
fn test() {
    let n = "43234".to_string();
    dbg!(Solution::nearest_palindromic(n));
}

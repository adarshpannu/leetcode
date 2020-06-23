const LOW: i32 = 1;
const HIGH: i32 = 7;

// Replace       a, b, c, d, e, f,   and   g       with the decimal digits   LOW   ───►   HIGH
// such that the sum of the letters inside of each of the four large squares add up to the same sum.

/*
a+b == b+c+d == d+e+f == f+g
*/

pub fn run() {
    let mut count_searched = 0u32;
    let mut count_selected = 0u32;

    for a in LOW..=HIGH {
        for b in LOW..=HIGH {
            if b == a {
                continue;
            }
            for c in LOW..=HIGH {
                if c == a || c == b {
                    continue;
                }
                for d in LOW..=HIGH {
                    if d == a || d == b || d == c || (a != c + d) {
                        continue;
                    }
                    for e in LOW..=HIGH {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        for f in LOW..=HIGH {
                            if f == a || f == b || f == c || f == d || f == e || (b + c != e + f) {
                                continue;
                            }
                            for g in LOW..=HIGH {
                                if g == a || g == b || g == c || g == d || g == e || g == f {
                                    continue;
                                }
                                count_searched += 1;
                                if d + e == g {
                                    println!("{} {} {} {} {} {} {}", a, b, c, d, e, f, g);
                                    count_selected += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("searched = {}, selected = {}", count_searched, count_selected);
}

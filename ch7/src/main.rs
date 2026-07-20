fn detect_capital_use(word: String) -> bool {
    let mut chars = word.chars();

    let Some(first) = chars.next() else {
        return false;
    };
    let first_upper = first.is_uppercase();

    match chars.next() {
        None => true,
        Some(second) => {
            let second_upper = second.is_uppercase();

            if first_upper && second_upper {
                chars.all(|c| c.is_uppercase())
            } else if first_upper && !second_upper {
                chars.all(|c| c.is_lowercase())
            } else {
                !second_upper && chars.all(|c| c.is_lowercase())
            }
        }
    }
}

fn license_key_formatting(s: String, k: i32) -> String {
    let k = k as usize;
    let mut result = String::new();
    let mut count = 0;

    for c in s.chars().rev() {
        if c == '-' {
            continue;
        }

        if count == k {
            result.push('-');
            count = 0;
        }
        result.push(c.to_ascii_uppercase());
        count += 1;
    }
    result.chars().rev().collect()
}
struct Solution;

impl Solution {
    pub fn mask_pii(s: String) -> String {
        if s.contains('@') {
            Self::mask_email(s)
        } else {
            Self::maks_phone(s)
        }
    }

    fn mask_email(s: String) -> String {
        let s = s.to_ascii_lowercase();
        let (name, domain) = s.split_once('@').unwrap();
        let mut result = String::with_capacity(s.len() + 5);
        let chars: Vec<char> = name.chars().collect();
        result.push(chars[0]);
        result.push_str("******");
        result.push(*chars.last().unwrap());
        result.push('@');
        result.push_str(domain);
        result
    }

    fn maks_phone(s: String) -> String {
        let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();

        let country_len = digits.len() - 10;

        let mut result = String::new();

        if country_len > 0 {
            result.push('+');
            result.extend(std::iter::repeat("*").take(country_len));
            result.push('-');
        }
        result.push_str("***-***-");
        result.push_str(&digits[digits.len() - 4..]);
        result
    }
}

fn main() {
    let word = String::from("USA");
    let result = detect_capital_use(word);
    println!("result: {}", result);

    let s = String::from("5F3Z-2e-9-w");
    let k = 4;
    let result = license_key_formatting(s, k);
    println!("result: {}", result);

    let s = String::from("LeetCode@LeetCode.com");
    let result = Solution::mask_pii(s);
    println!("result: {}", result);
}

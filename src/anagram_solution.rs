// 检测两个字符串是否为乱序字符串
use std::collections::HashMap;

pub fn anagram_solution(s1: &str, s2: &str) -> bool {
    if s1.chars().count() != s2.chars().count() {
        return false;
    }

    let mut s1_dict = HashMap::new();
    for c in s1.chars() {
        if let Some(value) = s1_dict.get_mut(&c) {
            *value += 1;
        } else {
            s1_dict.insert(c, 1);
        }
    }

    let mut ok = true;

    for c in s2.chars() {
        if let Some(value) = s1_dict.get_mut(&c) {
            *value -= 1;
            if *value < 0 {
                ok = false;
                break;
            }
        } else {
            ok = false;
            break;
        }
    }

    ok
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn anagram_solution_test() {
        assert_eq!(anagram_solution("aabcdefg", "bcdefaga"), true);
        assert_eq!(anagram_solution("aabcdefg", "aaaddefg"), false);
        assert_eq!(anagram_solution("abcdefg", "fgedcba"), true);
    }
}

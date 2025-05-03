fn anagram_checker(s1: &str, s2: &str) -> bool {
    if s1 == s2 {
        return true;
    }

    let mut s1_chars: Vec<_> = s1.chars().collect();
    let mut s2_chars: Vec<_> = s2.chars().collect();
    s1_chars.sort();
    s2_chars.sort();
    s1_chars == s2_chars
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("Usage: {} <string1> <string2>", args[0]);
        std::process::exit(1);
    }
    let s1 = &args[1];
    let s2 = &args[2];
    if anagram_checker(s1, s2) {
        println!("\"{}\" and \"{}\" are anagrams.", s1, s2);
    } else {
        println!("\"{}\" and \"{}\" are not anagrams.", s1, s2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_checker() {
        assert_eq!(anagram_checker("listen", "silent"), true);
        assert_eq!(anagram_checker("triangle", "integral"), true);
        assert_eq!(anagram_checker("apple", "pale"), false);
        assert_eq!(anagram_checker("abc", "cba"), true);
        assert_eq!(anagram_checker("abcd", "abcde"), false);
        assert_eq!(anagram_checker("aabbcc", "abcabc"), true);
        assert_eq!(anagram_checker("aabbcc", "aabbcc"), true);
    }

    #[test]
    fn test_anagram_checker_empty() {
        assert_eq!(anagram_checker("", ""), true);
        assert_eq!(anagram_checker("a", ""), false);
        assert_eq!(anagram_checker("", "a"), false);
    }

    #[test]
    fn test_anagram_checker_special_chars() {
        assert_eq!(anagram_checker("a!b@c#", "#c@b!a"), true);
        assert_eq!(anagram_checker("123", "321"), true);
        assert_eq!(anagram_checker("abc123", "321cba"), true);
        assert_eq!(anagram_checker("abc", "123"), false);
    }

    #[test]
    fn test_anagram_checker_unicode() {
        assert_eq!(anagram_checker("あいうえお", "おえういあ"), true);
        assert_eq!(anagram_checker("漢字", "字漢"), true);
        assert_eq!(anagram_checker("漢字", "漢字"), true);
        assert_eq!(anagram_checker("漢字", "漢"), false);
    }
}
